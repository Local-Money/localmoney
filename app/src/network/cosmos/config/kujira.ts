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
  hubAddress: 'kujira1jycdc7nvsqe2u5w99lw5qetwfus5grrksh7q02h7mtrhwsf7zzlsa2nd6f',
  hubConfig: {
    offer_addr: 'kujira17yfjhj4adsqlsm452a4hawp6lgtzd2lyrqnmfe057vd5pcpd8rwsykv8na',
    trade_addr: 'kujira1kd0uf7za28hslrlz4ag82kyyaucmd8mzukut44p8nlnrkj4mps0sgxzuck',
    profile_addr: 'kujira1at4ls9w65hcpgk4jnzssdczyky2339h3apszcysh6xq9vx77mf8sdszr0s',
    trading_incentives_addr: 'kujira1lzsuzy7485zzyze0tla55vn4ddwxa2flwhws9mrwfxftmhzmnwuslxv6z2',
    local_market_addr: 'kujira1chejx4qqtvwxy6684yrsmf6pylancxqhk3vsmtleg5ta3zrffljq4xf685',
    local_denom: {
      native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local',
    },
  },
}
