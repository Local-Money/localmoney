import { TextDecoder, TextEncoder } from 'util'

import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import offers from './fixtures/offers.json'
import makerSecrets from './fixtures/maker_secrets.json'
import takerSecrets from './fixtures/taker_secrets.json'
import adminSecrets from './fixtures/admin_secrets.json'
import { getOrCreateOffer, setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import prices from './fixtures/update_prices.json'
import { decryptDataMocked, encryptDataMocked } from './helper'
import { DefaultError } from '~/network/chain-error'
import type { GetOffer, OfferResponse, PostOffer } from '~/types/components.interface'
import { FiatCurrency, TradeState } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let makerClient: TestCosmosChain
let takerClient: TestCosmosChain
let adminClient: TestCosmosChain
const takerContact = 'taker001'
const makerContact = 'maker001'
let tradeId = '0'

jest.setTimeout(60 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  makerClient = result.makerClient
  takerClient = result.takerClient
  adminClient = result.adminClient
})

offers[0].denom = { native: process.env.OFFER_DENOM! }
let myOffers: OfferResponse[] = []

describe('trade lifecycle happy path', () => {
  let requestedTradesCount = 0
  let releasedTradesCount = 0
  let offerResponse: OfferResponse
  it('should have an arbitrator available', async () => {
    const fiat = offers[0].fiat_currency as FiatCurrency
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
  // Create Offer
  it('should have an available offer', async () => {
    offerResponse = await getOrCreateOffer(makerClient)
    expect(offerResponse).toBeDefined()
    requestedTradesCount = offerResponse.profile.requested_trades_count
    releasedTradesCount = offerResponse.profile.released_trades_count
  })
  // Create Trade
  it('taker should create a trade', async () => {
    const offer = offerResponse.offer
    const taker_contact = await encryptDataMocked(offerResponse.profile.encryption_key!, takerContact)
    expect(offer).toHaveProperty('id')
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key: profile_taker_encrypt_key,
      taker_contact,
    })
    expect(tradeId).toBeDefined()
  })
  // Maker accepts the trade request
  it('maker should accept the trade request', async () => {
    let tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    const decryptedTakerContact = await decryptDataMocked(makerSecrets.privateKey, trade.seller_contact!)
    expect(decryptedTakerContact).toBe(takerContact)
    expect(trade.id).toBe(tradeId)
    expect(trade.state).toBe(TradeState.request_created)
    const maker_contact = await encryptDataMocked(trade.seller_encryption_key, makerContact)
    await makerClient.acceptTradeRequest(trade.id, maker_contact)
    tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.request_accepted)
  })
  // Taker funds the escrow
  it('taker should fund the escrow', async () => {
    const tradeAddr = makerClient.getHubInfo().hubConfig.trade_addr
    let tradeInfo = await takerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    const decryptedMakerContact = await decryptDataMocked(takerSecrets.privateKey, trade.buyer_contact!)
    expect(decryptedMakerContact).toBe(offers[0].owner_contact)
    const tradeBalance = (await makerClient.getCwClient().getBalance(tradeAddr, trade.denom.native)).amount
    await takerClient.fundEscrow(trade.id, trade.amount, trade.denom)
    const newTradeBalance = (await makerClient.getCwClient().getBalance(tradeAddr, trade.denom.native)).amount
    tradeInfo = await takerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.escrow_funded)
    expect(parseInt(newTradeBalance)).toBe(parseInt(tradeBalance) + parseInt(trade.amount) * 1.01)
  })
  it('maker should mark trade as paid (fiat_deposited)', async () => {
    await makerClient.setFiatDeposited(tradeId)
    const tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.fiat_deposited)
  })
  it('taker should release the trade.', async () => {
    await takerClient.releaseEscrow(tradeId)
    const tradeInfo = await takerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.escrow_released)
  })
  it('the trade count should increase after the release', async () => {
    const myOffers = await makerClient.fetchMyOffers()
    expect(myOffers[0].profile.requested_trades_count).toBe(requestedTradesCount + 1)
    expect(myOffers[0].profile.released_trades_count).toBe(releasedTradesCount + 1)
  })
})

describe('trade invalid state changes', () => {
  let requestedTradesCount = 0
  let releasedTradesCount = 0
  let offerResponse: OfferResponse
  it('should have an arbitrator available', async () => {
    offerResponse = await getOrCreateOffer(makerClient)
    expect(offerResponse).toBeDefined()
    requestedTradesCount = offerResponse.profile.requested_trades_count
    releasedTradesCount = offerResponse.profile.released_trades_count
    const fiat = offerResponse.offer.fiat_currency
    let arbitrators = await adminClient.fetchArbitrators()
    if (arbitrators.find((arbitrator) => arbitrator.fiat === fiat) === undefined) {
      await adminClient.newArbitrator({
        arbitrator: adminClient.getWalletAddress(),
        fiat,
        encryption_key: 'arbitrator_encrypt_public_key',
      })
      arbitrators = await adminClient.fetchArbitrators()
    }
    expect(arbitrators.filter((arb) => arb.fiat === fiat).length).toBeGreaterThan(0)
  })
  it('should fail to fund a trade in request_created state', async () => {
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const taker_encrypt_pk = takerSecrets.publicKey
    const taker_contact = await encryptDataMocked(offerResponse.profile.encryption_key!, takerContact)
    tradeId = await takerClient.openTrade({
      amount: offerResponse.offer.min_amount,
      offer_id: offerResponse.offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key: taker_encrypt_pk,
      taker_contact,
    })
    let tradeInfo = await takerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.request_created)
    // Taker tries to fund escrow before maker accepts the trade
    await expect(takerClient.fundEscrow(trade.id, trade.amount, trade.denom)).rejects.toThrow(DefaultError)
    tradeInfo = await takerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.request_created)
  })
  it('should fail to mark as paid a trade in request_created state', async () => {
    const tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.request_created)
    await expect(makerClient.setFiatDeposited(tradeId)).rejects.toThrow()
  })
  it('should fail to release or refund escrow of trade in request_created state', async () => {
    const tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.request_created)
    await expect(takerClient.releaseEscrow(tradeId)).rejects.toThrow()
    await expect(takerClient.refundEscrow(tradeId)).rejects.toThrow()
  })
  it('should fail to mark as paid a trade in request_accepted state', async () => {
    const offer = offers[0] as PostOffer
    let tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    const maker_contact = await encryptDataMocked(trade.seller_encryption_key, offer.owner_contact)
    await makerClient.acceptTradeRequest(trade.id, maker_contact)
    tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.request_accepted)
    await expect(takerClient.setFiatDeposited(tradeId)).rejects.toThrow()
  })
  it('should fail to release or refund a trade in request_accepted state', async () => {
    const tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.request_accepted)
    await expect(takerClient.releaseEscrow(tradeId)).rejects.toThrow()
    await expect(takerClient.refundEscrow(tradeId)).rejects.toThrow()
  })
  it('should fail to release or refund a trade in escrow_funded state', async () => {
    let tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    await takerClient.fundEscrow(trade.id, trade.amount, trade.denom)
    tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.escrow_funded)
  })
  it('should fail to cancel a trade in fiat_deposited state', async () => {
    await makerClient.setFiatDeposited(tradeId)
    const tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.fiat_deposited)
    await expect(makerClient.cancelTradeRequest(tradeId)).rejects.toThrow()
    expect(tradeInfo.trade.state).toBe(TradeState.fiat_deposited)
  })
  it('the trade count should not increase when a trade is not completed', async () => {
    const myOffers = await makerClient.fetchMyOffers()
    expect(myOffers[0].profile.requested_trades_count).toBe(requestedTradesCount + 1)
    expect(myOffers[0].profile.released_trades_count).toBe(releasedTradesCount)
  })
})

describe('trade with invalid price', () => {
  let requestedTradesCount = 0
  let offer: GetOffer
  it('should have an arbitrator available', async () => {
    const fiat = offers[0].fiat_currency as FiatCurrency
    let arbitrators = await adminClient.fetchArbitrators()
    if (arbitrators.find((arbitrator) => arbitrator.fiat === fiat) === undefined) {
      await adminClient.newArbitrator({
        arbitrator: adminClient.getWalletAddress(),
        fiat,
        encryption_key: 'arbitrator_encrypt_public_key',
      })
      arbitrators = await adminClient.fetchArbitrators()
    }
    expect(arbitrators.filter((arb) => arb.fiat === fiat).length).toBeGreaterThan(0)
  })
  it('should have an available offer', async () => {
    const offerResponse = await getOrCreateOffer(makerClient)
    expect(offerResponse).toBeDefined()
    offer = offerResponse.offer
    requestedTradesCount = offerResponse.profile.requested_trades_count
  })
  it('should not allow to create a trade when price for denom is zero', async () => {
    // Set price at Zero for Fiat Currency
    const priceAddr = takerClient.getHubInfo().hubConfig.price_addr
    const priceAtZero = {
      update_prices: [{ currency: offer.fiat_currency, usd_price: '0', updated_at: 0 }],
    }
    let update_price_result = await takerClient
      .getCwClient()
      .execute(takerClient.getWalletAddress(), priceAddr, priceAtZero, 'auto', 'register fiat prices at zero')
    expect(update_price_result.transactionHash).not.toBeNull()
    // Tries to create a trade
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    let tradeId = '0'
    await expect(async () => {
      tradeId = await takerClient.openTrade({
        amount: offer.min_amount,
        offer_id: offer.id,
        taker: takerClient.getWalletAddress(),
        profile_taker_contact,
        profile_taker_encryption_key: profile_taker_encrypt_key,
        taker_contact: 'taker_contact',
      })
    }).rejects.toThrow()
    expect(tradeId).toBe('0')
    myOffers = await makerClient.fetchMyOffers()
    expect(myOffers[0].profile.requested_trades_count).toStrictEqual(requestedTradesCount)
    await takerClient.connectWallet()
    // Fix price
    update_price_result = await takerClient
      .getCwClient()
      .execute(takerClient.getWalletAddress(), priceAddr, prices, 'auto', 'register fiat prices')
    expect(update_price_result.transactionHash).not.toBeNull()
  })
})

describe('test trade limits', () => {
  let offer: GetOffer
  it('should have an available offer', async () => {
    offer = (await getOrCreateOffer(makerClient)).offer
    expect(offer).toBeDefined()
  })
  it('should not allow a trade to be created above the limit', async () => {
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    // Try to create a trade with the amount above the limit
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    let tradeId = '0'
    // Query Price for Offer denom
    const usdPrice = await takerClient.fetchFiatPriceForDenom(FiatCurrency.USD, offer.denom)
    const fiatPriceDecimals = 100
    const price = usdPrice.price * (parseInt(offer.rate) / fiatPriceDecimals)
    const denomDecimals = 1_000_000
    const invalidAmount = (hubInfo.hubConfig.trade_limit / price) * denomDecimals * fiatPriceDecimals * 1.02 // 2% above the limit

    await expect(async () => {
      tradeId = await takerClient.openTrade({
        amount: invalidAmount.toFixed(0),
        offer_id: offer.id,
        taker: takerClient.getWalletAddress(),
        profile_taker_contact,
        profile_taker_encryption_key: profile_taker_encrypt_key,
        taker_contact: 'taker_contact',
      })
    }).rejects.toThrow(/Invalid trade amount/)
    expect(tradeId).toBe('0')
  })
})
