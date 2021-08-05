import argparse
from functools import partial
from pathlib import Path
import sqlite3.dbapi2 as sqlite3
from typing import Dict

from quart import Quart, g, render_template, request, redirect, url_for, session, flash
import pandas as pd

app = Quart(__name__)

app.config.update({
    'DATABASE': app.root_path / 'db.sqlite',
})


def connect_db():
    engine = sqlite3.connect(app.config['DATABASE'])
    engine.row_factory = sqlite3.Row
    return engine


def get_db():
    if not hasattr(g, 'sqlite_db'):
        g.sqlite_db = connect_db()
    return g.sqlite_db


@app.cli.command('init_db')
def init_db():
    db = connect_db()
    with open(Path(__file__).parent / 'schema_data.sql', mode='r') as file_:
        db.cursor().executescript(file_.read())
    with open(Path(__file__).parent / 'schema_scale.sql', mode='r') as file_:
        db.cursor().executescript(file_.read())
    db.commit()


@app.route('/', methods=['GET'])
async def entries():
    db = get_db()
    q1 = db.execute('SELECT * FROM data INNER JOIN scale s on s.id = data.type')
    data = q1.fetchall()

    q2 = db.execute('SELECT SUM(distance * s.scale) as sum FROM data INNER JOIN scale s on s.id = data.type')
    overall = q2.fetchone()

    return await render_template('entries.html', entries=data, overall=overall['sum'])


@app.route('/', methods=['POST'])
async def create():
    db = get_db()

    content = await request.files

    try:
        file = content['csvToUpload']
    except KeyError:
        return 'Error', 500

    data = pd.read_csv(file.stream, sep=',')

    q = db.execute('SELECT name, id FROM scale')
    activity_types = dict(q.fetchall())

    c = partial(activity_converter, activity_types)

    data['Activity ID'] = data.apply(c, axis='columns')

    for _, r in data.iterrows():
        db.execute('REPLACE INTO data (type, date, time, distance, elevation, title) VALUES (?, ?, ?, ?, ?, ?)',
                   (r['Activity ID'], r['Date'], r['Time'], r['Distance'], r['Elev Gain'], r['Title']))

    db.commit()

    return redirect(url_for('entries'))


def activity_converter(lookup: Dict[str, int], x: pd.Series) -> int:
    act_id = lookup[x['Activity Type']]
    # special handling for hiking, convert all with elevation gain < 300 to walking
    if x['Activity Type'] == 'Hiking' and float(x['Elev Gain']) < 300:
        act_id = lookup['Walking']

    return act_id


if __name__ == '__main__':
    app.run(debug=True)