import type { PluginOptions } from 'vue-toastification'
import Toast, { POSITION } from 'vue-toastification'
import type { UserModule } from '~/types'

const options: PluginOptions = {
  position: POSITION.TOP_CENTER,
  timeout: 5000,
  closeButton: 'button',
  icon: false,
  hideProgressBar: true,
  pauseOnFocusLoss: false,
  pauseOnHover: true,
  transition: 'Vue-Toastification__fade', // Vue-Toastification__bounce | Vue-Toastification__slideBlurred
  maxToasts: 5,
  newestOnTop: true,
  // toastDefaults: {
  //   [TYPE.ERROR]: {
  //     timeout: 10000,
  //   },
  //   [TYPE.WARNING]: {
  //     timeout: false,
  //   },
  // },
}
// https://github.com/Maronato/vue-toastification/
export const install: UserModule = ({ app, router, isClient }) => {
  if (isClient) {
    options.onMounted = (_, toastApp) => {
      // Register the router. See here https://github.com/Maronato/vue-toastification/issues/162#issuecomment-945208145
      toastApp.use(router)
    }
    app.use(Toast, options)
  }
}
