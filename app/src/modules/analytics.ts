import type { UserModule } from '~/types'
import { initAnalytics } from '~/analytics/analytics'

export const install: UserModule = () => {
  initAnalytics(process.env.ANALYTICS_TOKEN as string, { debug: true })
}
