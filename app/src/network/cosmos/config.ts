import type { HubConfig } from '~/types/components.interface'

export interface CosmosConfig {
  chainId: string
  chainName: string
  lcdUrl: string
  rpcUrl: string
  addressPrefix: string
  coinDenom: string
  coinMinimalDenom: string
  coinDecimals: number
}

export interface HubInfo {
  hubAddress: string
  hubConfig: HubConfig
}
