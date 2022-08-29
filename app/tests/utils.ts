export function createHubUpdateConfigMsg(offerAddr: string, tradeAddr: string, tradingIncentivesAddr: string) {
  return {
    update_config: {
      offer_addr: offerAddr,
      trade_addr: tradeAddr,
      trading_incentives_addr: tradingIncentivesAddr,
      local_market_addr: process.env.LOCAL_MARKET,
      local_denom: { native: process.env.LOCAL_DENOM },
      chain_fee_collector_addr: process.env.CHAIN_FEE_COLLECTOR,
      warchest_addr: process.env.WARCHEST_ADDR,
      warchest_fee_pct: '50',
      chain_fee_pct: '10',
      burn_fee_pct: '40',
    },
  }
}
