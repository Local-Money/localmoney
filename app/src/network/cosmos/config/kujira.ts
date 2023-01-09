import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'

export const KUJIRA_TESTNET_CONFIG: CosmosConfig = {
  chainId: 'kayio-1',
  chainName: 'Kujira',
  lcdUrl: 'https://lcd.kaiyo.kujira.setten.io/',
  rpcUrl: 'https://rpc.kaiyo.kujira.setten.io/',
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
