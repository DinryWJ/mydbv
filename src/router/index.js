import { createRouter, createWebHistory, createWebHashHistory  } from 'vue-router';
import Home from '../views/Home.vue';
import About from '../views/About.vue';
import NotFound from '../views/NotFound.vue';
import Query from '../views/Query.vue';
const routes = [
    {
        path: '/home',
        component: Home
    },
    {
        path: '/about',
        component: About
    },
    {
        path: '/query',
        component: Query
    },
    {
        path: '/:catchAll(.*)',
        component: NotFound
      },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;
