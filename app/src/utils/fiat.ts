import fiatList from './fiats-config.json'

export const fiatsAvailable = new Map<string, FiatInfo>(Object.entries(fiatList))

export function defaultFiatAvailable(): string {
  return fiatsAvailable.keys().next().value
}

export function checkFiatAvailable(microDenom: string): boolean {
  return fiatsAvailable.has(microDenom)
}

export function getFiatInfo(fiatCode: string): FiatInfo {
  return fiatsAvailable.get(fiatCode)!
}

interface FiatInfo {
  display: string
  flag: string
  code: string
}
