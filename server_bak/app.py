from flask import Flask, jsonify
from flask_cors import CORS
from datetime import datetime, timedelta


# configuration
DEBUG = True

# instantiate the app
app = Flask(__name__)
app.config.from_object(__name__)

# enable CORS
CORS(app, resources={r'/*': {'origins': '*'}})

DUMMY = [
        {
        'title': 'Activity 1',
        'type': 'Cycling',
        'date': datetime(2021, 7, 1, 10, 0, 0),
        'time': str(timedelta(hours=2)),
        'distance': 50,
        'elevation': 200,
        },
                {
        'title': 'Activity 2',
        'type': 'Hiking',
        'date': datetime(2021, 7, 2, 10, 0, 0),
        'time': str(timedelta(hours=3, minutes=30)),
        'distance': 5,
        'elevation': 500,
        }
]       


# sanity check route
@app.route('/ping', methods=['GET'])
def ping_pong():
        return jsonify('pong!')

@app.route('/activities', methods=['GET'])
def all_activities():
    return jsonify({
        'status': 'success',
        'activities': DUMMY
    })

@app.route('/overall', methods=['GET'])
def get_overall():
    return jsonify({
        'status': 'success',
        'overall': 123.456
    })


if __name__ == '__main__':
        app.run()
