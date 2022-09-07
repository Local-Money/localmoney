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
  hubAddress: 'kujira1jycdc7nvsqe2u5w99lw5qetwfus5grrksh7q02h7mtrhwsf7zzlsa2nd6f',
  hubConfig: {
    offer_addr: 'kujira17yfjhj4adsqlsm452a4hawp6lgtzd2lyrqnmfe057vd5pcpd8rwsykv8na',
    trade_addr: 'kujira1kd0uf7za28hslrlz4ag82kyyaucmd8mzukut44p8nlnrkj4mps0sgxzuck',
    trading_incentives_addr: 'kujira1lzsuzy7485zzyze0tla55vn4ddwxa2flwhws9mrwfxftmhzmnwuslxv6z2',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}

// TEST
export const TEST_CONFIG: CosmosConfig = {
  chainId: 'harpoon-2',
  chainName: 'Kujira Localnet',
  lcdUrl: 'http://172.17.229.197:1317',
  rpcUrl: 'http://172.17.229.197:26657',
  addressPrefix: 'kujira',
  coinDenom: 'KUJI',
  coinMinimalDenom: 'ukuji',
  coinDecimals: 6,
}

export const TEST_HUB_INFO: HubInfo = {
  hubAddress: process.env.HUB ? process.env.HUB : 'kujira122l9zsmswu5hl9kmalgyuxvyskt2ldrn62mlpenxae07pjwta3yqdr9nga',
  hubConfig: {
    offer_addr: '',
    trade_addr: '',
    trading_incentives_addr: '',
    local_market_addr: '',
    local_denom: {
      native: '',
    },
  },
}
