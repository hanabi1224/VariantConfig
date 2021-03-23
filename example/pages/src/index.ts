import Vue from 'vue';
import { Button, message } from 'ant-design-vue';
import 'ant-design-vue/dist/antd.css';
import AppPage from './App.vue';

// [Button,].forEach(e => {
//   Vue.use(e);
// });

// Vue.prototype.$message = message;

// Vue.config.productionTip = false;

new Vue({
  render: h => h(AppPage),
}).$mount('#app');
