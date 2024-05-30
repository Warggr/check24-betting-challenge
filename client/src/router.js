import { createRouter, createWebHistory } from 'vue-router'
import { Home, Communities, Betting, CommunityDashboard } from './views/index'
import store from './store'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL + "web/"),
  routes: [
    {
      path: '/',
      component: Home,
    },
    {
      path: '/communities',
      component: Communities,
      meta: { requiresAuth: true },
    },
    {
      path: '/bet',
      component: Betting,
    },
    {
      name: 'communityDashboard',
      path: '/communities/:id',
      component: CommunityDashboard,
      meta: { requiresAuth: true },
    },
    /*{
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue')
    }*/
  ]
})

router.beforeEach((to, _from) => {
  if (to.meta.requiresAuth && store.user === undefined) {
    return {
      path: '/login',
      query: { redirect: to.fullPath },
    }
  }
})

export default router
