import argparse
import sqlite3.dbapi2 as sqlite3
from pathlib import Path


def main(db_file: Path):
    db = sqlite3.connect(db_file)
    db.row_factory = sqlite3.Row

    with open(Path(__file__).parent / 'schema_scale.sql', mode='r') as file_:
        db.cursor().executescript(file_.read())

    with open(Path(__file__).parent / 'schema_data.sql', mode='r') as file_:
        db.cursor().executescript(file_.read())


if __name__ == '__main__':

    parser = argparse.ArgumentParser(description='Garmin Connect importer')
    # noinspection PyTypeChecker
    parser.add_argument('--db', type=Path, help='sqlite db', default=Path(__file__).parent / 'db.sqlite')

    args = parser.parse_args()

    main(args.db)