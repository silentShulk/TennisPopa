import { createRouter, createWebHistory } from 'vue-router'
import Home from '../pages/FormPage.vue'
import Page1 from '../pages/Giron.vue'
import Page2 from '../pages/Player.vue'

const routes = [
  {
    path: '/',
    name: 'FormPage',
    component: Home
  },
  {
    path: '/Giron',
    name: 'Giron',
    component: Page1
  },
  {
    path: '/Player',
    name: 'Player',
    component: Page2
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router