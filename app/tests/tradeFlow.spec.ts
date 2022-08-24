import { TextDecoder, TextEncoder } from 'util'

import type { InstantiateResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import dotenv from 'dotenv'
import { jest } from '@jest/globals'
import offers from './fixtures/offers.json'
import codeIds from './fixtures/codeIds.json'
import { createHubUpdateConfigMsg } from './utils/hub_utils'
import { DefaultError } from '~/network/chain-error'
import { ChainClient, chainFactory } from '~/network/Chain'
import type { TestCosmosChain } from '~/network/cosmos/TestCosmosChain'
import type { GetOffer, PostOffer } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let makerClient: TestCosmosChain
let takerClient: TestCosmosChain
let offersCount = 0
let tradeId = '0'

jest.setTimeout(30 * 1000)
beforeAll(async () => {
  makerClient = chainFactory(ChainClient.testCosmos) as TestCosmosChain
  makerClient.seed = process.env.TAKER_SEED!
  await makerClient.connectWallet()

  takerClient = chainFactory(ChainClient.testCosmos) as TestCosmosChain
  takerClient.seed = process.env.MAKER_SEED!
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
})

describe('trade lifecycle happy path', () => {
  // Create Offer
  if (process.env.CREATE_OFFERS) {
    it('should create offer', async () => {
      offersCount = (await makerClient.fetchMyOffers()).length
      await makerClient.createOffer(offers[0] as PostOffer)
    })
  }
  // Fetch Offers
  it('should fetch offers', async () => {
    const offersResult = await makerClient.fetchMyOffers()
    if (process.env.CREATE_OFFERS) {
      expect(offersResult.length).toBe(Math.min(offersCount + 1, 10))
    }
  })
  // Create Trade
  it('taker should create a trade', async () => {
    const createdOffer = offers[0] as PostOffer
    const offersResult = await makerClient.fetchOffers({
      denom: createdOffer.denom,
      fiatCurrency: createdOffer.fiat_currency,
      offerType: createdOffer.offer_type,
    })

    expect(offersResult.length).toBeGreaterThan(0)
    const offer = offersResult[0] as GetOffer
    expect(offer).toHaveProperty('id')

    const makerTradesCount = (await makerClient.fetchTrades()).length
    await takerClient.openTrade({ amount: offer.min_amount, offer_id: offer.id, taker: takerClient.getWalletAddress() })
    const trades = await makerClient.fetchTrades()
    const newMakerTradesCount = trades.length
    tradeId = trades[0].trade.id
    expect(newMakerTradesCount).toBeGreaterThan(makerTradesCount)
  })
  // Maker accepts the trade request
  it('maker should accept the trade request', async () => {
    let trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_created)
    await makerClient.acceptTradeRequest(trade.id)
    trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_accepted)
  })
  // Taker funds the escrow
  it('taker should fund the escrow', async () => {
    const tradeAddr = makerClient.getHubInfo().hubConfig.trade_addr
    let trade = await takerClient.fetchTradeDetail(tradeId)

    const tradeBalance = (await makerClient.getCwClient().getBalance(tradeAddr, trade.denom.native)).amount
    await takerClient.fundEscrow(trade.id, trade.amount, trade.denom)
    const newTradeBalance = (await makerClient.getCwClient().getBalance(tradeAddr, trade.denom.native)).amount
    trade = await takerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.escrow_funded)
    expect(parseInt(newTradeBalance)).toBe(parseInt(tradeBalance) + parseInt(trade.amount) * 1.01)
  })
  it('maker should mark trade as paid (fiat_deposited)', async () => {
    await makerClient.setFiatDeposited(tradeId)
    const trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.fiat_deposited)
  })
  it('taker should release the trade.', async () => {
    await takerClient.releaseEscrow(tradeId)
    const trade = await takerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.escrow_released)
  })
})

describe('trade invalid state changes', () => {
  it('should create offer', async () => {
    offersCount = (await makerClient.fetchMyOffers()).length
    await makerClient.createOffer(offers[0] as PostOffer)
  })
  it('should fail to fund a trade in request_created state', async () => {
    const offer = (await makerClient.fetchMyOffers())[0]
    await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
    })
    const tradeInfo = (await makerClient.fetchTrades())[0]
    let trade = tradeInfo.trade
    tradeId = trade.id
    expect(trade.state).toBe(TradeState.request_created)
    // Taker tries to fund escrow before maker accepts the trade
    await expect(takerClient.fundEscrow(trade.id, trade.amount, trade.denom)).rejects.toThrow(DefaultError)
    trade = await takerClient.fetchTradeDetail(tradeInfo.trade.id)
    expect(trade.state).toBe(TradeState.request_created)
  })
  it('should fail to mark as paid a trade in request_created state', async () => {
    const trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_created)
    await expect(makerClient.setFiatDeposited(tradeId)).rejects.toThrow()
  })
  it('should fail to release or refund escrow of trade in request_created state', async () => {
    const trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_created)
    await expect(takerClient.releaseEscrow(tradeId)).rejects.toThrow()
    await expect(takerClient.refundEscrow(tradeId)).rejects.toThrow()
  })
  it('should fail to mark as paid a trade in request_accepted state', async () => {
    await makerClient.acceptTradeRequest(tradeId)
    const trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_accepted)
    await expect(takerClient.setFiatDeposited(tradeId)).rejects.toThrow()
  })
  it('should fail to release or refund a trade in request_accepted state', async () => {
    const trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_accepted)
    await expect(takerClient.releaseEscrow(tradeId)).rejects.toThrow()
    await expect(takerClient.refundEscrow(tradeId)).rejects.toThrow()
  })
  it('should fail to release or refund a trade in escrow_funded state', async () => {
    let trade = await makerClient.fetchTradeDetail(tradeId)
    await takerClient.fundEscrow(trade.id, trade.amount, trade.denom)
    trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.escrow_funded)
  })
  it('should fail to cancel a trade in fiat_deposited state', async () => {
    await makerClient.setFiatDeposited(tradeId)
    const trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.fiat_deposited)
    await expect(makerClient.cancelTradeRequest(tradeId)).rejects.toThrow()
    expect(trade.state).toBe(TradeState.fiat_deposited)
  })
})
