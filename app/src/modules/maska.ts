import Maska from 'maska'
import { type UserModule } from '~/types'

export const install: UserModule = ({ app }) => {
  app.use(Maska)
}
