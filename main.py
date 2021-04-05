import argparse
from functools import partial
from pathlib import Path
import sqlite3.dbapi2 as sqlite3
from typing import Dict

import pandas as pd


def activity_converter(lookup: Dict[str, int], x: pd.Series) -> int:
    act_id = lookup[x['Activity Type']]
    # special handling for hiking, convert all with elevation gain < 300 to walking
    if x['Activity Type'] == 'Hiking' and float(x['Elev Gain']) < 300:
        act_id = lookup['Walking']

    return act_id


def main(i_file: Path, db_file: Path) -> None:
    db = sqlite3.connect(db_file)
    db.row_factory = sqlite3.Row
    data = pd.read_csv(i_file, sep=',')

    q = db.execute('SELECT name, id FROM scale')
    activity_types = dict(q.fetchall())

    c = partial(activity_converter, activity_types)

    data['Activity ID'] = data.apply(c, axis='columns')

    for _, r in data.iterrows():
        db.execute('REPLACE INTO data (type, date, distance, elevation, title) VALUES (?, ?, ?, ?, ?)',
                   (r['Activity ID'], r['Date'], r['Distance'], r['Elev Gain'], r['Title']))

    db.commit()

    pass


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Garmin Connect importer')
    # noinspection PyTypeChecker
    parser.add_argument('-i', '--input', type=Path, required=True, help='csv file exported from garmin connect')
    # noinspection PyTypeChecker
    parser.add_argument('--db', type=Path, help='sqlite db', default=Path(__file__).parent / 'db.sqlite')

    args = parser.parse_args()

    main(args.input, args.db)
