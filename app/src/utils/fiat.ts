import fiatList from './fiats-config.json'
import type { SelectInfo } from '~/utils/select-utils'

export const fiatsAvailable = new Map<string, FiatInfo>(Object.entries(fiatList))

export function checkFiatAvailable(microDenom: string): boolean {
  return fiatsAvailable.has(microDenom)
}

export function getFiatInfo(fiatCode: string): FiatInfo {
  return fiatsAvailable.get(fiatCode)!
}

interface FiatInfo extends SelectInfo {
  display: string
  icon: string
  code: string
}
