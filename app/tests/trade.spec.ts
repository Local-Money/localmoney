import { TextDecoder, TextEncoder } from 'util'

import dotenv from 'dotenv'
import { jest } from '@jest/globals'
import offers from './fixtures/offers.json'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import { DefaultError } from '~/network/chain-error'
import type { GetOffer, PostOffer } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let makerClient: TestCosmosChain
let takerClient: TestCosmosChain
let tradeId = '0'
let offerTradeCount = 0

jest.setTimeout(30 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  makerClient = result.makerClient
  takerClient = result.takerClient
})

offers[0].denom = { native: process.env.OFFER_DENOM! }

describe.skip('trade lifecycle happy path', () => {
  // Create Offer
  it('should have an available offer', async () => {
    let myOffers = await makerClient.fetchMyOffers()
    if (myOffers.length === 0) {
      await makerClient.createOffer(offers[0] as PostOffer)
    }
    myOffers = await makerClient.fetchMyOffers()
    offerTradeCount = myOffers[0].trades_count
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

    tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
    })
  })
  // Maker accepts the trade request
  it('maker should accept the trade request', async () => {
    const offer = offers[0] as PostOffer
    let trade = await makerClient.fetchTradeDetail(tradeId)
    expect(trade.id).toBe(tradeId)
    expect(trade.state).toBe(TradeState.request_created)
    await makerClient.acceptTradeRequest(trade.id, offer.owner_contact)
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
  it('the trade count should increase after the release', async () => {
    const myOffers = await makerClient.fetchMyOffers()
    expect(myOffers[0].trades_count).toBe(offerTradeCount + 1)
  })
})

describe.skip('trade invalid state changes', () => {
  it('should have an available offer', async () => {
    let myOffers = await makerClient.fetchMyOffers()
    if (myOffers.length === 0) {
      myOffers[0].denom = { native: process.env.OFFER_DENOM! }
      await makerClient.createOffer(offers[0] as PostOffer)
    }
    myOffers = await makerClient.fetchMyOffers()
    offerTradeCount = myOffers[0].trades_count
  })
  it('should fail to fund a trade in request_created state', async () => {
    const offer = (await makerClient.fetchMyOffers())[0]
    tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
    })
    let trade = await takerClient.fetchTradeDetail(tradeId)
    expect(trade.state).toBe(TradeState.request_created)
    // Taker tries to fund escrow before maker accepts the trade
    await expect(takerClient.fundEscrow(trade.id, trade.amount, trade.denom)).rejects.toThrow(DefaultError)
    trade = await takerClient.fetchTradeDetail(trade.id)
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
    const offer = offers[0] as PostOffer
    await makerClient.acceptTradeRequest(tradeId, offer.owner_contact)
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
  it('the trade count should not increase when a trade is not completed', async () => {
    const myOffers = await makerClient.fetchMyOffers()
    expect(myOffers[0].trades_count).toBe(offerTradeCount)
  })
})
