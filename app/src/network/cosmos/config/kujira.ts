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

export const KUJIRA_TESTNET_DEV_HUB_INFO: HubInfo = {
  hubAddress: 'kujira1vqpugrytms8pd8mjkyamd506954vrujv0zl20klej7p7trddc74s05vedm',
  hubConfig: {
    offer_addr: 'kujira1z4gfgy2a2sj5ewanay3z4k5hkglh6y2a9p9t87ntrnfu70ld443stktjqf',
    trade_addr: 'kujira1fwlm7qy2x4zh8tu042t8e2kvc33w7yt0usj6dqm4m0q6mcujwj6qlvjldx',
    price_addr: 'kujira12lz7vskwhwt0m0gakn9sdmgrdrr5xet6nd47snualrh05kw2lfxqm9u699',
    profile_addr: 'kujira10pws69q67cst5x22la0yj5h3s09vexppumw4emgcavc9w9arreaqh9da3t',
    trading_incentives_addr: 'kujira1kmwgx6cvr7t6sjhh3443rmy248psfnqr9cgd0lsarxh59rsj3hysnh956g',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}
