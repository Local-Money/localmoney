import { config, project_key } from './analytics.json'
import type { UserModule } from '~/types'
import { initAnalytics } from '~/analytics/analytics'

export const install: UserModule = () => {
  initAnalytics(project_key, config)
}
