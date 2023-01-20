import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'

export const DEV_CONFIG: CosmosConfig = {
  chainId: process.env.CHAIN_ID ?? '',
  chainName: process.env.CHAIN_NAME ?? '',
  lcdUrl: process.env.LCD ?? '',
  rpcUrl: process.env.RPC ?? '',
  addressPrefix: 'kujira',
  coinDenom: 'KUJI',
  coinMinimalDenom: 'ukuji',
  coinDecimals: 6,
}

export const DEV_HUB_INFO: HubInfo = {
  hubAddress: process.env.HUB ?? '',
  hubConfig: {
    offer_addr: process.env.OFFER ?? '',
    trade_addr: process.env.TRADE ?? '',
    profile_addr: '',
    local_market_addr: process.env.LOCAL_MARKET ?? '',
    local_denom: {
      native: process.env.LOCAL_DENOM ?? '',
    },
  },
}
