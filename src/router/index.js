import { createRouter, createWebHistory } from 'vue-router'
import Home from '../pages/FormPage.vue'
import Page1 from '../pages/Groups.vue'
import Page2 from '../pages/Player.vue'
import Page3 from '../pages/GetFormResponsesPage.vue'
import Page4 from '../pages/Schedule.vue'

const routes = [
  {
    path: '/',
    name: 'FormPage',
    component: Home
  },
  {
    path: '/Groups',
    name: 'Groups',
    component: Page1
  },
  {
    path: '/Player',
    name: 'Player',
    component: Page2
  },
  {
    path: '/GetFormResponses',
    name: 'GetFormResponses',
    component: Page3
  },
  {
    path: '/Schedule',
    name: 'Schedule',
    component: Page4
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router