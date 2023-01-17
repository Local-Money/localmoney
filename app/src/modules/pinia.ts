import { createPinia } from 'pinia'
import type { Router } from 'vue-router'
import type { ToastInterface } from 'vue-toastification'
import { useToast } from 'vue-toastification'
import { type UserModule } from '~/types'

// Setup Pinia
// https://pinia.esm.dev/

declare module 'pinia' {
  export interface PiniaCustomProperties {
    router: Router
    toast: ToastInterface
  }
}

export const install: UserModule = ({ isClient, initialState, app }) => {
  const pinia = createPinia()
  pinia.use(({ store }) => {
    const router = useRouter()
    const toast = useToast()
    store.router = markRaw(router)
    store.toast = markRaw(toast)
  })
  app.use(pinia)
  // Refer to
  // https://github.com/antfu/vite-ssg/blob/main/README.md#state-serialization
  // for other serialization strategies.
  if (isClient) {
    pinia.state.value = initialState.pinia || {}
  } else {
    initialState.pinia = pinia.state.value
  }
}
