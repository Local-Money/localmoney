import denomList from './denoms-config.json'

export const denomsAvailable = new Map<string, Denom>(Object.entries(denomList))

export function defaultMicroDenomAvailable(): string {
  return denomsAvailable.keys().next().value
}

export function checkMicroDenomAvailable(microDenom: string): boolean {
  return denomsAvailable.has(microDenom)
}

export function denomFromMicroDenom(microDenom: string): string {
  return denomsAvailable.has(microDenom) ? denomsAvailable.get(microDenom)!.denom : microDenom
}

interface Denom {
  denom: string
}
