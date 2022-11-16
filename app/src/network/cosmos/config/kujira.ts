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
    offer_addr: 'kujira149hxq2d852xtkp3tgcn2jvfvrajqrah95ytxe8umsdr8nlw8cagqcd472u',
    trade_addr: 'kujira1gyesrk08chq7au849sv6gx87guy9vr2dlx68xxv2ej66l70mrznsex9g6z',
    profile_addr: 'kujira1xp4ynuqsqffdwgytfujq7amkxzs7h230yuhlncd9ulkqfvc9thwsfgrrn5',
    price_addr: 'kujira1deldrarr6uryhhw9hfgaxtyhzm9lkvztp6qpvy9ws5lzuhwpd9dsv9ngxc',
    trading_incentives_addr: 'kujira1n8sr7ndqvfl5uxp3xumtte3pyza8dpc9saq6ypalatvrcjqd54vstr72tp',
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
  hubAddress: 'kujira1jqy9qfatg9jz9gkl6dyrmc4z8hjfaujgf2qujshe0mnsf6yajess8wxnp3',
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
