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
    profile_addr: 'kujira1at4ls9w65hcpgk4jnzssdczyky2339h3apszcysh6xq9vx77mf8sdszr0s',
    trading_incentives_addr: 'kujira1c75t034k4tr43dzcw9dyym480p6r3kwdyr5ecnafvrat9gn4n2us83frqq',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}
