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
  hubAddress: 'kujira16u6leuaassucdvx6awuzxypp0r7wdusl8rmm72f689f8qcvd2p4q5pznz3',
  hubConfig: {
    offer_addr: 'kujira1nhr7kxps88vvy6tz6sx7d4x0j0gnl95e5s05feengutvmcl7nyws7sg9ce',
    trade_addr: 'kujira1y8artn4enfulxlau9ycv33e8xrsy6a29n20dl6pfhxgflfqd005qp0vj6n',
    price_addr: 'kujira14e8dc9x2vqltcs28wwu96jtnymrse80cmmj9nc37x4ymh7xjmrjqavedqd',
    profile_addr: 'kujira190ea5krlq7gvdalsaay8u2ww23v9y6qs9qvyljh2s0x6h8qpkvwq4sgc7c',
    trading_incentives_addr: 'kujira1e3zup87nv4uqfk2vruhsp6nfhm2ep6xf0erzgwxessuqdamp93gqtlrtec',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}
