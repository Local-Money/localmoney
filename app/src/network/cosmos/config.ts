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

// JUNO
export const JUNO_CONFIG: CosmosConfig = {
  chainId: 'uni-3',
  chainName: 'Juno Testnet',
  lcdUrl: 'https://lcd.uni.juno.deuslabs.fi/',
  rpcUrl: 'https://rpc.uni.juno.deuslabs.fi/',
  addressPrefix: 'juno',
  coinDenom: 'JUNOX',
  coinMinimalDenom: 'ujunox',
  coinDecimals: 6,
}

export const JUNO_HUB_INFO: HubInfo = {
  hubAddress: 'juno1mf8mc3d5k4mny5djsuqsp7t9p9dmjxaj4gky4q0lthx6zrmjrlyq2yknmd',
  hubConfig: {
    offer_addr: 'juno1qzjsgnpwtlz265t2cxheftf5mlgs7lqjacvz3lkp5vr4ga3rgassgvsegx',
    trade_addr: 'juno1htpc5ek2p80agv2x7tjgx6l03p52l930dd4dyk3qk9raanzyrngq4zx8da',
    trading_incentives_addr: 'juno1a6qzj0ep50jjn97ufmsjlwdhnqmny2phhcug3e6404tkngfd906syquknw',
    local_market_addr: 'juno1gqhxtrsve4f2pcp65fr8l5t86pu7v0cxqvqgj6',
    local_denom: { native: 'ujunox' },
  },
}

// KUJIRA
export const KUJIRA_CONFIG: CosmosConfig = {
  chainId: 'harpoon-4',
  chainName: 'Kujira Testnet',
  lcdUrl: 'https://lcd.harpoon.kujira.setten.io/',
  rpcUrl: 'https://rpc.harpoon.kujira.setten.io/',
  addressPrefix: 'kujira',
  coinDenom: 'KUJI',
  coinMinimalDenom: 'ukuji',
  coinDecimals: 6,
}

export const KUJIRA_HUB_INFO: HubInfo = {
  hubAddress: 'kujira13zjt2swjk0un2fpp3259szed7dsfmv3etdfkumrstlrdcq3szx9sew48gr',
  hubConfig: {
    offer_addr: 'kujira1cryjuex5uxstsstr8j50qpm4g9828nlr0l5uunlwvj0e7umdv7yq2m4rrt',
    trade_addr: 'kujira1kl8fyh4245lqj3ht5cl099eswuhgds4k9zdfv6hrhf3xu8xplzyqs8vklx',
    trading_incentives_addr: 'kujira1t7tjl3fegh8m9yn37eks2m2qaelcz2kuvyyzpp28vcx42vame65sevwhhh',
    local_market_addr: 'kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w',
    local_denom: {
      native: 'factory/kujira16g2rahf5846rxzp3fwlswy08fz8ccuwkgthh5j/local',
    },
  },
}
