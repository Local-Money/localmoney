import denomList from './denoms-config.json'
import type { SelectInfo } from '~/utils/select-utils'
import type { CW20, Denom, Native } from '~/types/components.interface'
import { ChainClient } from '~/network/Chain'

export function denomsAvailable(chainClient: ChainClient): Map<string, MicronDenom> {
  switch (chainClient) {
    case ChainClient.kujiraMainnet:
      return new Map<string, MicronDenom>(Object.entries(denomList.kujira_mainnet))
    default:
      return new Map<string, MicronDenom>(Object.entries(denomList.kujira_testnet))
  }
}

export function defaultMicroDenomAvailable(chainClient: ChainClient): string {
  const denoms = denomsAvailable(chainClient)
  return denoms.keys().next().value
}

export function checkMicroDenomAvailable(microDenom: string, chainClient: ChainClient): boolean {
  const denoms = denomsAvailable(chainClient)
  return denoms.has(microDenom)
}

export function microDenomToDisplay(microDenom: string, chainClient: ChainClient): string {
  const denoms = denomsAvailable(chainClient)
  return denoms.has(microDenom) ? denoms.get(microDenom)!.display : microDenom
}

export function denomToValue(denom: Denom): string {
  return (denom as Native).native ?? (denom as CW20).cw20
}

interface MicronDenom extends SelectInfo {
  display: string
  icon: string
}
