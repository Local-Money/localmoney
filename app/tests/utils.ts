import type { InstantiateResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { TestCosmosChain } from './network/TestCosmosChain'
import codeIds from './fixtures/codeIds.json'
import { TEST_CONFIG, TEST_HUB_INFO } from '~/network/cosmos/config'

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

export async function setupProtocol() {
  const makerClient = new TestCosmosChain(TEST_CONFIG, TEST_HUB_INFO)
  makerClient.seed = process.env.MAKER_SEED!
  await makerClient.connectWallet()

  const takerClient = new TestCosmosChain(TEST_CONFIG, TEST_HUB_INFO)
  takerClient.seed = process.env.TAKER_SEED!
  await takerClient.connectWallet()

  if (process.env.HUB) {
    await makerClient.updateHub(process.env.HUB)
    await takerClient.updateHub(process.env.HUB)
  } else {
    // Instantiate all contracts
    const walletAddress = makerClient.getWalletAddress()
    const cwClient = makerClient.getCwClient() as SigningCosmWasmClient

    const instantiateMsg = { admin_addr: walletAddress }
    const { hub, offer, trade, trading_incentives } = codeIds
    const hubInstantiateResult = await cwClient.instantiate(walletAddress, hub, instantiateMsg, 'hub', 'auto')
    const offerInstantiateResult = await cwClient.instantiate(walletAddress, offer, instantiateMsg, 'offer', 'auto')
    const tradeInstantiateResult = await cwClient.instantiate(walletAddress, trade, instantiateMsg, 'trade', 'auto')
    const tradingIncentivesResult = await cwClient.instantiate(
      walletAddress,
      trading_incentives,
      instantiateMsg,
      'trading_incentives',
      'auto'
    )

    // Assert that all contracts were instantiated
    const results = [hubInstantiateResult, offerInstantiateResult, tradeInstantiateResult, tradingIncentivesResult]
    results.forEach((result: InstantiateResult) => {
      expect(result).toHaveProperty('contractAddress')
    })

    // Setup Hub
    const updatedConfigMsg = createHubUpdateConfigMsg(
      offerInstantiateResult.contractAddress,
      tradeInstantiateResult.contractAddress,
      tradingIncentivesResult.contractAddress
    )
    await cwClient.execute(walletAddress, hubInstantiateResult.contractAddress, updatedConfigMsg, 'auto')
    await makerClient.updateHub(hubInstantiateResult.contractAddress)
    await takerClient.updateHub(hubInstantiateResult.contractAddress)
    expect(makerClient.getHubInfo().hubConfig.trade_addr).toBe(tradeInstantiateResult.contractAddress)
  }
  return { makerClient, takerClient }
}
