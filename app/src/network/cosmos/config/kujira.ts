import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'

export const KUJIRA_TESTNET_CONFIG: CosmosConfig = {
  chainId: 'harpoon-4',
  chainName: 'Kujira Testnet',
  lcdUrl: 'https://test-lcd-kujira.mintthemoon.xyz/',
  rpcUrl: 'https://test-rpc-kujira.mintthemoon.xyz/',
  addressPrefix: 'kujira',
  coinDenom: 'KUJI',
  coinMinimalDenom: 'ukuji',
  coinDecimals: 6,
}

export const KUJIRA_TESTNET_HUB_INFO: HubInfo = {
  hubAddress: 'kujira1vyy7f920fm4rrcwgzac4p0grez2e5ckd4zjq7v9ahumcpre4ft5qu7agt7',
  hubConfig: {
    offer_addr: 'kujira1h7g5u3lr5j74hcsfrrls4anauv7u72a94cppyscl32rqm5lzz7psu4438v',
    trade_addr: 'kujira13lg50j2hak2quzax2xvt6sjk6j7s73ljzvvrv95cjjcur95z62cstl3qqj',
    price_addr: 'kujira14e8dc9x2vqltcs28wwu96jtnymrse80cmmj9nc37x4ymh7xjmrjqavedqd',
    profile_addr: 'kujira15felz5ycysspattde8sl26wqw0m9ghzgg7dajm6edt96a03sjw8qxk92fq',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'ukuji',
    },
  },
}

export const KUJIRA_MAINNET_CONFIG: CosmosConfig = {
  chainId: 'kaiyo-1',
  chainName: 'Kujira',
  lcdUrl: 'https://lcd-kujira.mintthemoon.xyz/',
  rpcUrl: 'https://rpc-kujira.mintthemoon.xyz/',
  addressPrefix: 'kujira',
  coinDenom: 'KUJI',
  coinMinimalDenom: 'ukuji',
  coinDecimals: 6,
}

export const KUJIRA_MAINNET_HUB_INFO: HubInfo = {
  hubAddress: 'kujira1392dk7n69wj2gvz25ygtyz48w0lgtkrgay72c60vszhrrkkrtw9qzced2h',
  hubConfig: {
    offer_addr: 'kujira1hpmasdua44u83h497p6wpu84dmmvafj82xd4jfky7z55vpf8a4rqd7qeat',
    trade_addr: 'kujira1hdydzhe7dfhw2vmfsrzu6dcw2aeuff4ja8wr5puqhsg8rlf44gvq6car5s',
    price_addr: 'kujira189qa4s2c8kyhcrf5e75fhmaa6g8ux2h76p06qx32cxgasy07mtqq9mudvh',
    profile_addr: 'kujira1tyqttr06k5hgrz95a508mu5euvjadtfyfaf4yu2hnh6tueswkvwsjy7290',
    local_market_addr: 'kujira1sse6a00arh9dalzsyrd3q825dsn2zmrag0u4qx8q0dyks4ftnxyqrj0xds',
    local_denom: {
      native: 'factory/kujira1swkuyt08z74n5jl7zr6hx0ru5sa2yev5v896p6/local',
    },
  },
}
