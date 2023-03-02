import denomList from './denoms-config.json'
import type { SelectInfo } from '~/utils/select-utils'
import type { CW20, Denom, Native } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { ChainClient } from '~/network/Chain'

export function denomsAvailable(): Map<string, MicronDenom> {
  const chainClient = useClientStore().chainClient
  switch (chainClient) {
    case ChainClient.kujiraMainnet:
      return new Map<string, MicronDenom>(Object.entries(denomList.kujira_mainnet))
    default:
      return new Map<string, MicronDenom>(Object.entries(denomList.kujira_testnet))
  }
}

export function defaultMicroDenomAvailable(): string {
  const denoms = denomsAvailable()
  return denoms.keys().next().value
}

export function checkMicroDenomAvailable(microDenom: string): boolean {
  const denoms = denomsAvailable()
  return denoms.has(microDenom)
}

export function microDenomToDenom(microDenom: string): string {
  const denoms = denomsAvailable()
  return denoms.has(microDenom) ? denoms.get(microDenom)!.display : microDenom
}

export function denomToValue(denom: Denom): string {
  return (denom as Native).native ?? (denom as CW20).cw20
}

interface MicronDenom extends SelectInfo {
  display: string
  icon: string
}
