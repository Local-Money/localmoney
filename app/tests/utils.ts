import type { InstantiateResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { TestCosmosChain } from './network/TestCosmosChain'
import codeIds from './fixtures/codeIds.json'
import { DEV_CONFIG, DEV_HUB_INFO } from '~/network/cosmos/config/dev'

export function createHubUpdateConfigMsg(
  offerAddr: string,
  tradeAddr: string,
  tradingIncentivesAddr: string,
  profileAddr: string
) {
  return {
    update_config: {
      offer_addr: offerAddr,
      trade_addr: tradeAddr,
      trading_incentives_addr: tradingIncentivesAddr,
      profile_addr: profileAddr,
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
  const adminClient = new TestCosmosChain(DEV_CONFIG, DEV_HUB_INFO)
  adminClient.seed = process.env.ADMIN_SEED!
  await adminClient.connectWallet()

  const makerClient = new TestCosmosChain(DEV_CONFIG, DEV_HUB_INFO)
  makerClient.seed = process.env.MAKER_SEED!
  await makerClient.connectWallet()

  const takerClient = new TestCosmosChain(DEV_CONFIG, DEV_HUB_INFO)
  takerClient.seed = process.env.TAKER_SEED!
  await takerClient.connectWallet()

  if (process.env.HUB) {
    await adminClient.updateHub(process.env.HUB)
    await makerClient.updateHub(process.env.HUB)
    await takerClient.updateHub(process.env.HUB)
  } else {
    // Instantiate all contracts
    const admAddr = adminClient.getWalletAddress()
    const adminCwClient = adminClient.getCwClient() as SigningCosmWasmClient

    const instantiateMsg = { admin_addr: admAddr }
    const { hub, offer, trade, trading_incentives, profile } = codeIds
    const opts = { admin: admAddr }
    const hubInstantiateResult = await adminCwClient.instantiate(admAddr, hub, instantiateMsg, 'hub', 'auto', opts)
    const offerInstantiateResult = await adminCwClient.instantiate(
      admAddr,
      offer,
      instantiateMsg,
      'offer',
      'auto',
      opts
    )
    const tradeInstantiateResult = await adminCwClient.instantiate(
      admAddr,
      trade,
      instantiateMsg,
      'trade',
      'auto',
      opts
    )
    const tradingIncentivesResult = await adminCwClient.instantiate(
      admAddr,
      trading_incentives,
      instantiateMsg,
      'trading_incentives',
      'auto',
      opts
    )
    const profileResult = await adminCwClient.instantiate(admAddr, profile, instantiateMsg, 'profile', 'auto', opts)

    // Assert that all contracts were instantiated
    const results = [
      hubInstantiateResult,
      offerInstantiateResult,
      tradeInstantiateResult,
      tradingIncentivesResult,
      profileResult,
    ]
    results.forEach((result: InstantiateResult) => {
      expect(result).toHaveProperty('contractAddress')
    })

    // Setup Hub
    const updatedConfigMsg = createHubUpdateConfigMsg(
      offerInstantiateResult.contractAddress,
      tradeInstantiateResult.contractAddress,
      tradingIncentivesResult.contractAddress,
      profileResult.contractAddress
    )
    await adminCwClient.execute(admAddr, hubInstantiateResult.contractAddress, updatedConfigMsg, 'auto')
    await adminClient.updateHub(hubInstantiateResult.contractAddress)
    await makerClient.updateHub(hubInstantiateResult.contractAddress)
    await takerClient.updateHub(hubInstantiateResult.contractAddress)
    expect(adminClient.getHubInfo().hubConfig.trade_addr).toBe(tradeInstantiateResult.contractAddress)
    expect(makerClient.getHubInfo().hubConfig.trade_addr).toBe(tradeInstantiateResult.contractAddress)
    expect(takerClient.getHubInfo().hubConfig.trade_addr).toBe(tradeInstantiateResult.contractAddress)
    // eslint-disable-next-line no-console
    console.log('Admin Wallet: ', adminClient.getWalletAddress())
    console.log('Maker Wallet: ', makerClient.getWalletAddress())
    console.log('Taker Wallet: ', takerClient.getWalletAddress())
    console.log('Hub Address:', hubInstantiateResult.contractAddress)
  }
  return { adminClient, makerClient, takerClient }
}
