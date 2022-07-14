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
  hubAddress: 'juno10xspvwdnzqxqgfu30hzz3q8ne0qzj88kvnp75grqhcks3hs4wqzqehw9d3',
  hubConfig: {
    offer_addr: 'juno1k55r769kf0wrmt333s0l0gwv3eflyxwtkeaqvnpzhmag5kk39mnse0tg5f',
    trade_addr: 'juno1vr7tkvcqftncc2crt68mxg6uhqk84660gwy4rjh3udajhv9s4pgshfx4qh',
    trading_incentives_addr: 'juno1cwxxtrq8nqlpacx2uky768r3c9vghh5suzgzw50uy87ne77p5kvs6496sy',
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
