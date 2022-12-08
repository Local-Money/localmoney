import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'

export const JUNO_TESTNET_CONFIG: CosmosConfig = {
  chainId: 'uni-3',
  chainName: 'Juno Testnet',
  lcdUrl: 'https://lcd.uni.juno.deuslabs.fi/',
  rpcUrl: 'https://rpc.uni.juno.deuslabs.fi/',
  addressPrefix: 'juno',
  coinDenom: 'JUNOX',
  coinMinimalDenom: 'ujunox',
  coinDecimals: 6,
}

export const JUNO_TESTNET_HUB_INFO: HubInfo = {
  hubAddress: 'juno1mf8mc3d5k4mny5djsuqsp7t9p9dmjxaj4gky4q0lthx6zrmjrlyq2yknmd',
  hubConfig: {
    offer_addr: 'juno1qzjsgnpwtlz265t2cxheftf5mlgs7lqjacvz3lkp5vr4ga3rgassgvsegx',
    trade_addr: 'juno1htpc5ek2p80agv2x7tjgx6l03p52l930dd4dyk3qk9raanzyrngq4zx8da',
    profile_addr: '',
    local_market_addr: 'juno1gqhxtrsve4f2pcp65fr8l5t86pu7v0cxqvqgj6',
    local_denom: { native: 'ujunox' },
  },
}
