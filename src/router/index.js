import { createRouter, createWebHistory } from 'vue-router'
import Home from '../components/Home.vue'
import Offer from '../components/Offer.vue'
import Trade from '../components/Trade.vue'
import CreateOffer from '../components/CreateOffer.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/newoffer',
    name: 'NewOffer',
    component: CreateOffer,
  },
  {
    path: '/offer/:id',
    name: 'Offer',
    component: Offer,
  },
  {
    path: '/trade/:id',
    name: 'Trade',
    component: Trade,
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

export default router
