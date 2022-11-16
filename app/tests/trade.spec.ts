import { TextDecoder, TextEncoder } from 'util'

import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import offers from './fixtures/offers.json'
import makerSecrets from './fixtures/maker_secrets.json'
import takerSecrets from './fixtures/taker_secrets.json'
import adminSecrets from './fixtures/admin_secrets.json'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import { decryptDataMocked, encryptDataMocked } from './helper'
import { DefaultError } from '~/network/chain-error'
import type { FiatCurrency, OfferResponse, PostOffer } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'

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

describe.only('trade lifecycle happy path', () => {
  let requestedTradesCount = 0
  let releasedTradesCount = 0
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
    myOffers = await makerClient.fetchMyOffers()
    if (myOffers.length === 0) {
      const owner_contact = await encryptDataMocked(makerSecrets.publicKey, makerContact)
      const owner_encryption_key = makerSecrets.publicKey
      const offer = { ...offers[0], owner_contact, owner_encryption_key } as PostOffer
      await makerClient.createOffer(offer)
    }
    myOffers = await makerClient.fetchMyOffers()
    requestedTradesCount = myOffers[0].profile.requested_trades_count
    releasedTradesCount = myOffers[0].profile.released_trades_count
    expect(myOffers.length).toBeGreaterThan(0)
  })
  // Create Trade
  it('taker should create a trade', async () => {
    const offerResponse = myOffers[0] as OfferResponse
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
    console.log('Trade Id:', tradeId)
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
    let myOffers = await makerClient.fetchMyOffers()
    if (myOffers.length === 0) {
      const owner_contact = await encryptDataMocked(makerSecrets.publicKey, makerContact)
      const owner_encryption_key = makerSecrets.publicKey
      const denom = { native: process.env.OFFER_DENOM! }
      const newOffer = { ...offers[0], owner_contact, owner_encryption_key, denom } as PostOffer
      await makerClient.createOffer(newOffer)
    }
    myOffers = await makerClient.fetchMyOffers()
    requestedTradesCount = myOffers[0].profile.requested_trades_count
    releasedTradesCount = myOffers[0].profile.released_trades_count
  })
  it('should fail to fund a trade in request_created state', async () => {
    const offerResponse = myOffers[0] as OfferResponse
    const offer = offerResponse.offer
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const taker_encrypt_pk = takerSecrets.publicKey
    const taker_contact = await encryptDataMocked(offerResponse.profile.encryption_key!, takerContact)
    tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
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
