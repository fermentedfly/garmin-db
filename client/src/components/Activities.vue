<template>
  <div class="container">
    <div class="row">
      <div class="col-sm-10">
          <h1>Overall</h1>
          <p>{{overall}}</p>
      </div>
    </div>
    <div class="row">
      <div class="col-sm-10">
        <h1>Activities</h1>
        <hr />
        <br /><br />
        <button type="button" class="btn btn-success btn-sm">
          Upload new activities
        </button>
        <br /><br />
        <table class="table table-hover">
          <thead>
            <tr>
              <th scope="col">Title</th>
              <th scope="col">Type</th>
              <th scope="col">Date</th>
              <th scope="col">Time</th>
              <th scope="col">Distance</th>
              <th scope="col">Elevation</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(activity, index) in activities" :key="index">
              <td>{{activity.title}}</td>
              <td>{{activity.type}}</td>
              <td>{{activity.date}}</td>
              <td>{{activity.time}}</td>
              <td>{{activity.distance}}</td>
              <td>{{activity.elevation}}</td>
              <td>
                <div class="btn-group" role="group">
                  <button type="button" class="btn btn-success btn-sm">
                    Details
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      activities: [],
      overall: 0.0,
    };
  },
  methods: {
    getActivities() {
      const path = 'http://localhost:5000/activities';
      axios.get(path)
        .then((res) => {
          this.activities = res.data.activities;
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
    getOverall() {
      const path = 'http://localhost:5000/overall';
      axios.get(path)
        .then((res) => {
          this.overall = res.data.overall;
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
  },
  created() {
    this.getActivities();
    this.getOverall();
  },
};
</script>
