import { TextDecoder, TextEncoder } from 'util'

import dotenv from 'dotenv'
import { jest } from '@jest/globals'
import offers from './fixtures/offers.json'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import type { GetOffer, PostOffer } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let makerClient: TestCosmosChain
let takerClient: TestCosmosChain

jest.setTimeout(30 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  makerClient = result.makerClient
  takerClient = result.takerClient
})

describe('arbitration tests', () => {
  // Call dispute_escrow on a trade in fiat_deposited state and expects it to be in escrow_disputed state
  it('should dispute escrow of a trade', async () => {
    // Create and fetch offer for trade creation
    offers[0].denom = { native: process.env.OFFER_DENOM! }
    if (process.env.CREATE_OFFERS) {
      await makerClient.createOffer(offers[0] as PostOffer)
    }
    const createdOffer = offers[0] as PostOffer
    const offersResult = await makerClient.fetchOffers({
      denom: createdOffer.denom,
      fiatCurrency: createdOffer.fiat_currency,
      offerType: createdOffer.offer_type,
    })

    // Create a Trade and set it to `fiat_deposited` state.
    const offer = offersResult[0] as GetOffer
    await takerClient.openTrade({ amount: offer.min_amount, offer_id: offer.id, taker: takerClient.getWalletAddress() })
    const tradeInfo = (await makerClient.fetchTrades())[0]
    await makerClient.acceptTradeRequest(tradeInfo.trade.id)
    await takerClient.fundEscrow(tradeInfo.trade.id, tradeInfo.trade.amount, tradeInfo.trade.denom)
    await makerClient.setFiatDeposited(tradeInfo.trade.id)

    let trade = await takerClient.fetchTradeDetail(tradeInfo.trade.id)
    expect(trade.state).toBe(TradeState.fiat_deposited)
    await takerClient.openDispute(trade.id)
    trade = await takerClient.fetchTradeDetail(tradeInfo.trade.id)
    expect(trade.state).toBe(TradeState.escrow_disputed)
  })
})
