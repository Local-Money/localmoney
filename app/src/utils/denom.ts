import denomList from './denoms-config.json'
import type { SelectInfo } from '~/utils/select-utils'

export const denomsAvailable = new Map<string, MicronDenom>(Object.entries(denomList))

export function defaultMicroDenomAvailable(): string {
  return denomsAvailable.keys().next().value
}

export function checkMicroDenomAvailable(microDenom: string): boolean {
  return denomsAvailable.has(microDenom)
}

export function microDenomToDenom(microDenom: string): string {
  return denomsAvailable.has(microDenom) ? denomsAvailable.get(microDenom)!.display : microDenom
}

interface MicronDenom extends SelectInfo {
  display: string
}
