import { TextDecoder, TextEncoder } from 'util'
import dotenv from 'dotenv'
import { jest } from '@jest/globals'
import offers from './fixtures/offers.json'
import { setupProtocol, sleep } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import { encryptDataMocked } from './helper'
import takerSecrets from './fixtures/taker_secrets.json'
import makerSecrets from './fixtures/maker_secrets.json'
import adminSecrets from './fixtures/admin_secrets.json'
import { TRADE_DISPUTE_TIMER } from './configs'
import type { FiatCurrency, GetOffer, OfferResponse, PostOffer } from '~/types/components.interface'
import { OfferOrder, TradeState } from '~/types/components.interface'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let makerClient: TestCosmosChain
let takerClient: TestCosmosChain
let adminClient: TestCosmosChain

const takerContact = 'taker001'
const makerContact = 'maker001'

jest.setTimeout(3 * 60 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  makerClient = result.makerClient
  takerClient = result.takerClient
  adminClient = result.adminClient
})

let offer: GetOffer
let offerResponse: OfferResponse[]

describe('arbitration tests', () => {
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
  // Call dispute_escrow on a trade in fiat_deposited state and expects it to be in escrow_disputed state
  it('should have available offers', async () => {
    // Create and fetch offer for trade creation
    const owner_contact = await encryptDataMocked(makerSecrets.publicKey, makerContact)
    const owner_encryption_key = makerSecrets.publicKey
    const denom = { native: process.env.OFFER_DENOM! }
    const createdOffer = { ...offers[0], owner_contact, owner_encryption_key, denom } as PostOffer
    if (process.env.CREATE_OFFERS) {
      await makerClient.createOffer(createdOffer)
    }
    offerResponse = await makerClient.fetchOffers({
      denom: createdOffer.denom,
      fiatCurrency: createdOffer.fiat_currency,
      offerType: createdOffer.offer_type,
      order: OfferOrder.trades_count,
    })
    offer = offerResponse[0].offer as GetOffer
    expect(offer.id).not.toBeNaN()
  })

  it('should settle dispute for taker', async () => {
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encryption_key = takerSecrets.publicKey
    const taker_contact = await encryptDataMocked(offerResponse[0].profile.encryption_key!, takerContact)
    // Create a Trade and set it to `fiat_deposited` state.
    const tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key,
      taker_contact,
    })

    let tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    const makerContactEncrypted = await encryptDataMocked(trade.seller_encryption_key, makerContact)
    await makerClient.acceptTradeRequest(tradeId, makerContactEncrypted)
    await takerClient.fundEscrow(tradeInfo)
    await makerClient.setFiatDeposited(trade.id)
    tradeInfo = await takerClient.fetchTradeDetail(trade.id)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.fiat_deposited)

    // Wait the time to enable dispute
    await sleep((TRADE_DISPUTE_TIMER + 1) * 1000)
    // Taker disputes the escrow
    const buyer_contact = await encryptDataMocked(makerContact, trade.arbitrator_encryption_key)
    const seller_contact = await encryptDataMocked(taker_contact, trade.arbitrator_encryption_key)
    await takerClient.openDispute(trade.id, buyer_contact, seller_contact)
    tradeInfo = await takerClient.fetchTradeDetail(trade.id)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.escrow_disputed)

    // Arbitrator settles for Taker
    await adminClient.settleDispute(trade.id, takerClient.getWalletAddress())
    tradeInfo = await takerClient.fetchTradeDetail(trade.id)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.settled_for_taker)
  })

  it('should settle dispute for maker', async () => {
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const taker_encrypt_pk = takerSecrets.publicKey
    const taker_contact = await encryptDataMocked(offerResponse[0].profile.encryption_key!, takerContact)

    const tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key: taker_encrypt_pk,
      taker_contact,
    })

    let tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    let trade = tradeInfo.trade
    const makerContactEncrypted = await encryptDataMocked(trade.seller_encryption_key, makerContact)
    await makerClient.acceptTradeRequest(tradeId, makerContactEncrypted)
    await takerClient.fundEscrow(tradeInfo, trade.amount)
    await makerClient.setFiatDeposited(trade.id)
    // Wait the time to enable dispute
    await sleep((TRADE_DISPUTE_TIMER + 1) * 1000)
    // Taker disputes the escrow
    await takerClient.openDispute(trade.id, 'buyer_contact', 'seller_contact')
    tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.escrow_disputed)

    // Arbitrator settles for Maker
    await adminClient.settleDispute(trade.id, makerClient.getWalletAddress())
    tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.settled_for_maker)
  })

  it('should not be able to open a dispute prematurely', async () => {
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const taker_encrypt_pk = takerSecrets.publicKey
    const taker_contact = await encryptDataMocked(offerResponse[0].profile.encryption_key!, takerContact)

    const tradeId = await takerClient.openTrade({
      amount: offer.min_amount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key: taker_encrypt_pk,
      taker_contact,
    })

    const tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    const trade = tradeInfo.trade
    const makerContactEncrypted = await encryptDataMocked(trade.seller_encryption_key, makerContact)
    await makerClient.acceptTradeRequest(tradeId, makerContactEncrypted)
    await takerClient.fundEscrow(tradeInfo, trade.amount)
    await makerClient.setFiatDeposited(trade.id)
    // Tries to open dispute prematurely
    await expect(takerClient.openDispute(trade.id, 'buyer_contact', 'seller_contact')).rejects.toThrow()
  })

  it('should query trades by arbitrator', async () => {
    const tradesByArbitrator = await takerClient
      .getCwClient()
      .queryContractSmart(takerClient.getHubInfo().hubConfig.trade_addr, {
        trades: {
          user: adminClient.getWalletAddress(),
          role: 'arbitrator',
          limit: 10,
        },
      })
    expect(tradesByArbitrator.length).toBeGreaterThan(0)
  })

  it('should remove arbitrator', async () => {
    const adminCwClient = adminClient.getCwClient()
    const adminAddr = adminClient.getWalletAddress()
    const tradeAddr = adminClient.getHubInfo().hubConfig.trade_addr
    let arbitrators = await adminCwClient.queryContractSmart(tradeAddr, {
      arbitrators: {
        limit: 10,
      },
    })
    expect(arbitrators.length).toBe(1)
    await adminCwClient.execute(
      adminAddr,
      tradeAddr,
      {
        delete_arbitrator: {
          arbitrator: adminAddr,
          fiat: offer.fiat_currency,
        },
      },
      'auto'
    )
    arbitrators = await adminCwClient.queryContractSmart(tradeAddr, {
      arbitrators: {
        limit: 10,
      },
    })
    expect(arbitrators.length).toBe(0)
  })
})
