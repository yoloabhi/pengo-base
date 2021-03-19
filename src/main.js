import Vue from 'vue';
import VueRouter from 'vue-router';
import RouterPrefetch from 'vue-router-prefetch'
import DashboardPlugin from './plugins/dashboard-plugin';
import App from './App.vue';

// router setup
import router from './routes/router';
import i18n from './i18n';
import './registerServiceWorker'
// plugin setup
Vue.use(DashboardPlugin);
Vue.use(VueRouter);
Vue.use(RouterPrefetch);

import { initContract } from "./utils"
Vue.config.productionTip = false

/* eslint-disable no-new */
window.nearInitPromise = initContract()
  .then(() => {
    new Vue({
      el: '#app',
      render: h => h(App),
      router,
      i18n
    });
  })
