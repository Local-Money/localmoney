import { TextDecoder, TextEncoder } from 'util'
import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import makerSecrets from './fixtures/maker_secrets.json'
import takerSecrets from './fixtures/taker_secrets.json'
import { encryptDataMocked } from './helper'
import adminSecrets from './fixtures/admin_secrets.json'
import type { PatchOffer } from '~/types/components.interface'
import { FiatCurrency, OfferOrder, OfferState, OfferType, TradeState } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

const args = {
  denom: { native: process.env.OFFER_DENOM! },
  fiatCurrency: FiatCurrency.ARS,
  offerType: OfferType.sell,
  order: OfferOrder.trades_count,
}
const limit = 3

let adminClient: TestCosmosChain
let takerClient: TestCosmosChain
let makerClient: TestCosmosChain
jest.setTimeout(60 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  adminClient = result.adminClient
  takerClient = result.takerClient
  makerClient = result.makerClient
})

describe('offers pagination', () => {
  it(`maker must have at least ${limit} offers`, async () => {
    let myOffers = await makerClient.fetchMyOffers(limit)
    myOffers = myOffers.filter((offer) => offer.offer.state !== OfferState.archived)
    const ownerContact = await encryptDataMocked(makerSecrets.publicKey, 'maker001')
    if (myOffers.length < limit) {
      for (let i = 0; i < limit - myOffers.length; i++) {
        await makerClient.createOffer({
          denom: args.denom,
          fiat_currency: args.fiatCurrency,
          min_amount: '120000',
          max_amount: '5000000',
          offer_type: args.offerType,
          owner_contact: ownerContact,
          owner_encryption_key: makerSecrets.publicKey,
          rate: (100 - i).toString(),
        })
      }
      myOffers = await makerClient.fetchMyOffers(limit)
    }
    expect(myOffers.length).toBe(limit)
  })
  it('maker should be able to paginate the list of offers', async () => {
    const myOffers = await makerClient.fetchMyOffers(limit)
    for (let i = 0; i < myOffers.length - 1; i++) {
      const last = myOffers[i].offer.id
      const nextPage = await makerClient.fetchMyOffers(1, last)
      expect(nextPage[0].offer.id).toBe(myOffers[i + 1].offer.id)
    }
  })
  it(`taker should list only ${limit} offers`, async () => {
    const offers = await takerClient.fetchOffers(args, limit)
    expect(offers.length).toBe(limit)
  })
  it('taker should be able to paginate the list of offers', async () => {
    let offers = await takerClient.fetchOffers(args, limit)
    for (let i = 0; i < offers.length - 1; i++) {
      const previousId = offers[offers.length - 2].offer.id
      const last = offers[offers.length - 1].offer.id
      offers = await takerClient.fetchOffers(args, limit, last)
      expect(offers[offers.length - 1].offer.id).toBe(previousId)
    }
  })
})

describe('trades pagination', () => {
  it('should have an arbitrator available', async () => {
    const fiat = args.fiatCurrency
    let arbitrators = await adminClient.fetchArbitrators()
    if (arbitrators.find((arbitrator) => arbitrator.fiat === fiat) === undefined) {
      await adminClient.newArbitrator({
        arbitrator: adminClient.getWalletAddress(),
        fiat,
        encryption_key: adminSecrets.publicKey,
      })
      arbitrators = await adminClient.fetchArbitrators()
    }
    expect(arbitrators.filter((arb) => arb.fiat === fiat).length).toBeGreaterThan(0)
  })
  it(`maker must have at least ${limit} trades`, async () => {
    let trades = await makerClient.fetchTrades()
    if (trades.length < limit) {
      const offers = await takerClient.fetchOffers(args, 1)
      const offerResponse = offers[0]
      const profileTakerContact = await encryptDataMocked(takerSecrets.publicKey, 'taker001')
      const takerContact = await encryptDataMocked(offerResponse.profile.encryption_key!, 'taker001')
      for (let i = 0; i < limit - trades.length; i++) {
        await takerClient.openTrade({
          amount: offerResponse.offer.min_amount,
          offer_id: offerResponse.offer.id,
          taker: takerClient.getWalletAddress(),
          profile_taker_contact: profileTakerContact,
          profile_taker_encryption_key: takerSecrets.publicKey,
          taker_contact: takerContact,
        })
      }
      trades = await makerClient.fetchTrades()
    }
    expect(trades.length).toBeGreaterThanOrEqual(limit)
  })
  it('maker should be able to paginate the list of trades', async () => {
    const trades = await makerClient.fetchTrades(limit)
    for (let i = 0; i < trades.length - 1; i++) {
      const last = trades[i].trade.id
      const nextPage = await makerClient.fetchMyOffers(1, last)
      expect(nextPage[0].offer.id).toBe(trades[i + 1].trade.id)
    }
  })
  it('taker should be able to paginate the list of trades', async () => {
    const trades = await takerClient.fetchTrades(limit)
    for (let i = 0; i < trades.length - 1; i++) {
      const last = trades[i].trade.id
      const nextPage = await makerClient.fetchMyOffers(1, last)
      expect(nextPage[0].offer.id).toBe(trades[i + 1].trade.id)
    }
  })
})

afterAll(async () => {
  // close all open trades in the trades pagination test
  let trades = await makerClient.fetchTrades(limit)
  trades = trades.filter((trade) => trade.trade.state === TradeState.request_created)
  for (const tradeInfo of trades) {
    await makerClient.cancelTradeRequest(tradeInfo.trade.id)
  }

  // archive all offers created in the offers pagination test
  let offers = await makerClient.fetchMyOffers(limit)
  offers = offers.filter((offer) => offer.offer.state !== OfferState.archived)
  for (const offerResponse of offers) {
    const offer: PatchOffer = {
      ...offerResponse.offer,
      state: OfferState.archived,
    }
    await makerClient.updateOffer(offer)
  }
})
