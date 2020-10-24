import { createApp, h } from 'vue';
import IndexPage from './pages/Index.vue';
import AboutPage from './pages/About.vue';

import './main.css';

const routes = {
  '/': IndexPage,
  '/about': AboutPage,
};

const SimpleRouter = {
  data: () => ({
    currentRoute: window.location.pathname,
  }),

  computed: {
    CurrentComponent() {
      return routes[this.currentRoute] || IndexPage;
    },
  },

  render() {
    return h(this.CurrentComponent);
  },
};

createApp(SimpleRouter).mount('#app');
