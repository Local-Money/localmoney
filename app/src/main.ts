import { ViteSSG } from 'vite-ssg'
import { createWebHistory } from 'vue-router'
import App from './App.vue'
import routes from '~/routes'

// https://github.com/antfu/vite-ssg
export const createApp = ViteSSG(
  App,
  {
    routes,
    base: import.meta.env.BASE_URL,
    history: createWebHistory(process.env.BASE_URL),
  },
  (ctx) => {
    // install all modules under `modules/`
    Object.values(import.meta.globEager('./modules/*.ts')).forEach((i) => i.install?.(ctx))
  }
)
