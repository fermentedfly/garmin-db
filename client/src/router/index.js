import Vue from 'vue';
import VueRouter from 'vue-router';
import Activities from '../components/Activities.vue';

Vue.use(VueRouter);

export default new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'Activities',
      component: Activities,
    },
  ],
});
