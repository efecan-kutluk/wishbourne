/**
 * router/index.ts
 *
 * Automatic routes for `./src/pages/*.vue`
 */

// Composables
import Login from '@/pages/Login.vue'
import index from '@/pages/index.vue'
import { createRouter, createWebHistory } from 'vue-router/auto'
const routes = [
  { path: '/', component: index },
  { path: '/auth', component: Login },
]
const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  ...routes
})

export default router
