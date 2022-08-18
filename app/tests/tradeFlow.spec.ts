import { TextDecoder, TextEncoder } from 'util'

import type { InstantiateResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import dotenv from 'dotenv'
import { jest } from '@jest/globals'
import offers from './fixtures/offers.json'
import codeIds from './fixtures/codeIds.json'
import { createHubUpdateConfigMsg } from './utils/hub_utils'
import { ChainClient, chainFactory } from '~/network/Chain'
import type { TestCosmosChain } from '~/network/cosmos/TestCosmosChain'
import type { GetOffer, PostOffer } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let client: TestCosmosChain
let offersCount = 0
describe('Trade Lifecycle Happy Path', () => {
  jest.setTimeout(30 * 1000)
  beforeAll(async () => {
    client = chainFactory(ChainClient.testCosmos) as TestCosmosChain
    await client.connectWallet()

    if (process.env.HUB) {
      await client.updateHub(process.env.HUB)
    }
    else {
      // Instantiate all contracts
      const walletAddress = client.getWalletAddress()
      const cwClient = client.getCwClient() as SigningCosmWasmClient

      const instantiateMsg = { admin_addr: walletAddress }
      const hubInstantiateResult = await cwClient.instantiate(walletAddress, codeIds.hub, instantiateMsg, 'hub', 'auto')
      const offerInstantiateResult = await cwClient.instantiate(walletAddress, codeIds.offer, instantiateMsg, 'offer', 'auto')
      const tradeInstantiateResult = await cwClient.instantiate(walletAddress, codeIds.trade, instantiateMsg, 'trade', 'auto')
      const tradingIncentivesInstantiateResult = await cwClient.instantiate(walletAddress, codeIds.trading_incentives, instantiateMsg, 'trading_incentives', 'auto')

      // Assert that all contracts were instantiated
      const results = [hubInstantiateResult, offerInstantiateResult, tradeInstantiateResult, tradingIncentivesInstantiateResult]
      results.forEach((result: InstantiateResult) => {
        expect(result).toHaveProperty('contractAddress')
      })

      // Setup Hub
      const updatedConfigMsg = createHubUpdateConfigMsg(offerInstantiateResult.contractAddress,
        tradeInstantiateResult.contractAddress, tradingIncentivesInstantiateResult.contractAddress)
      await cwClient.execute(walletAddress, hubInstantiateResult.contractAddress, updatedConfigMsg, 'auto')
      await client.updateHub(hubInstantiateResult.contractAddress)
      expect(client.getHubInfo().hubConfig.trade_addr).toBe(tradeInstantiateResult.contractAddress)
    }
  })
  // Create Offer
  if (process.env.CREATE_OFFERS) {
    it('should create offer', async () => {
      offersCount = (await client.fetchMyOffers()).length
      await client.createOffer(offers[0] as PostOffer)
    })
  }
  // Fetch Offers
  it('should fetch offers', async () => {
    const offersResult = await client.fetchMyOffers()
    if (process.env.CREATE_OFFERS)
      expect(offersResult.length).toBe(Math.min(offersCount + 1, 10))
  })
  // Create Trade
  it('should create a trade', async () => {
    const createdOffer = offers[0] as PostOffer
    const offersResult = await client.fetchOffers({
      denom: createdOffer.denom, fiatCurrency: createdOffer.fiat_currency, offerType: createdOffer.offer_type,
    })

    expect(offersResult.length).toBeGreaterThan(0)
    const offer = offersResult[0] as GetOffer
    expect(offer).toHaveProperty('id')

    await client.openTrade({ amount: offer.min_amount, offer_id: offer.id, taker: client.getWalletAddress() })
    await client.fetchTrades()
  })
})
