import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'

export const KUJIRA_TESTNET_CONFIG: CosmosConfig = {
  chainId: 'harpoon-4',
  chainName: 'Kujira Testnet',
  lcdUrl: 'https://lcd.harpoon.kujira.setten.io/',
  rpcUrl: 'https://rpc.harpoon.kujira.setten.io/',
  addressPrefix: 'kujira',
  coinDenom: 'KUJI',
  coinMinimalDenom: 'ukuji',
  coinDecimals: 6,
}

export const KUJIRA_TESTNET_HUB_INFO: HubInfo = {
  hubAddress: 'kujira1fmhvwkzaxnwtjj0pzawclr8eh5twe32vgc272lxqun883wpzkn9squu6zz',
  hubConfig: {
    offer_addr: 'kujira1x42zrgk0l2k2fqyue2zyhwdky8ajy9d05u6p93yktf5g5pkvr9js4zdtdx',
    trade_addr: 'kujira1kgzvfedtcv57a2yj4fhsckhfu75z0e0lz8dkku2054ry79sf770s4pv3nu',
    profile_addr: 'kujira1pa8l5y9khqlueqq8nqdr485atq978nchwj8dc35rmvwg7hnsuzrshgk06g',
    price_addr: 'kujira1eg7ldgu78g7j9y6kcmms6acw9a6n2z4zun6prgkddped4x27vsgqgmxfcq',
    trading_incentives_addr: 'kujira1c75t034k4tr43dzcw9dyym480p6r3kwdyr5ecnafvrat9gn4n2us83frqq',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}

// TODO: Change price_addr after https://blue.kujira.app/govern/95 passes on testnet
// the contract must be migrated first.
// price_addr: 'kujira1z7hlmqyx52mup4gxuspvj2yvyf9kqkektu8e06upuf6qf7yvrgfqrerugc',
export const KUJIRA_TESTNET_DEV_HUB_INFO: HubInfo = {
  hubAddress: 'kujira1jycdc7nvsqe2u5w99lw5qetwfus5grrksh7q02h7mtrhwsf7zzlsa2nd6f',
  hubConfig: {
    offer_addr: 'kujira17yfjhj4adsqlsm452a4hawp6lgtzd2lyrqnmfe057vd5pcpd8rwsykv8na',
    trade_addr: 'kujira1kd0uf7za28hslrlz4ag82kyyaucmd8mzukut44p8nlnrkj4mps0sgxzuck',
    price_addr: 'kujira1lzsuzy7485zzyze0tla55vn4ddwxa2flwhws9mrwfxftmhzmnwuslxv6z2',
    profile_addr: 'kujira1at4ls9w65hcpgk4jnzssdczyky2339h3apszcysh6xq9vx77mf8sdszr0s',
    trading_incentives_addr: 'kujira1lzsuzy7485zzyze0tla55vn4ddwxa2flwhws9mrwfxftmhzmnwuslxv6z2',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}
