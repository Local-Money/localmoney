import { TextDecoder, TextEncoder } from 'util'

import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import offers from './fixtures/offers.json'
import makerSecrets from './fixtures/maker_secrets.json'
import takerSecrets from './fixtures/taker_secrets.json'
import adminSecrets from './fixtures/admin_secrets.json'
import { createHubUpdateConfigMsg, getOrCreateOffer, setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import prices from './fixtures/update_prices.json'
import { decryptDataMocked, encryptDataMocked } from './helper'
import { DefaultError } from '~/network/chain-error'
import type { GetOffer, OfferResponse, PostOffer, TradeInfo } from '~/types/components.interface'
import { FiatCurrency, OfferState, OfferType, TradeState } from '~/types/components.interface'
import { denomToValue } from '~/utils/denom'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let makerClient: TestCosmosChain
let takerClient: TestCosmosChain
let adminClient: TestCosmosChain
const takerContact = 'taker001'
const makerContact = 'maker001'
let tradeId = 0

jest.setTimeout(60 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  makerClient = result.makerClient
  takerClient = result.takerClient
  adminClient = result.adminClient
})

offers[0].denom = { native: process.env.OFFER_DENOM! }
let myOffers: OfferResponse[] = []

async function getValidTradeAmount(client: TestCosmosChain, offer: GetOffer): Promise<string> {
  const hubInfo = client.getHubInfo()
  const usdPrice = await client.updateFiatPrice(FiatCurrency.USD, offer.denom)
  const fiatPriceDecimals = 100
  const price = usdPrice.price * (parseInt(offer.rate) / fiatPriceDecimals)
  const denomDecimals = 1_000_000
  const tradeAmount = (hubInfo.hubConfig.trade_limit_min / price) * denomDecimals * fiatPriceDecimals * 1.01
  console.log('tradeAmount', tradeAmount)
  return Math.floor(tradeAmount).toFixed(0)
}

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
    const tradeAmount = await getValidTradeAmount(takerClient, offer)

    tradeId = await takerClient.openTrade({
      amount: tradeAmount,
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
    const tradeBalance = (await makerClient.getCwClient().getBalance(tradeAddr, denomToValue(trade.denom))).amount
    await takerClient.fundEscrow(tradeInfo)
    const newTradeBalance = (await makerClient.getCwClient().getBalance(tradeAddr, denomToValue(trade.denom))).amount
    tradeInfo = await takerClient.fetchTradeDetail(tradeId)
    trade = tradeInfo.trade
    expect(trade.state).toBe(TradeState.escrow_funded)
    console.log('trade.state', trade.state)
    expect(parseInt(newTradeBalance)).toBe(parseInt(tradeBalance) + parseInt(trade.amount))
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
    // Fetch Offer and USD price for denom
    const offer = offerResponse.offer
    const tradeAmount = await getValidTradeAmount(takerClient, offer)

    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const taker_encrypt_pk = takerSecrets.publicKey
    const taker_contact = await encryptDataMocked(offerResponse.profile.encryption_key!, takerContact)
    tradeId = await takerClient.openTrade({
      amount: tradeAmount,
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
    await expect(takerClient.fundEscrow(tradeInfo)).rejects.toThrow(DefaultError)
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
    await takerClient.fundEscrow(tradeInfo)
    tradeInfo = await makerClient.fetchTradeDetail(tradeId)
    expect(tradeInfo.trade.state).toBe(TradeState.escrow_funded)
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
    const tradeAmount = await getValidTradeAmount(takerClient, offer)
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    let tradeId = 0
    await expect(async () => {
      tradeId = await takerClient.openTrade({
        amount: tradeAmount,
        offer_id: offer.id,
        taker: takerClient.getWalletAddress(),
        profile_taker_contact,
        profile_taker_encryption_key: profile_taker_encrypt_key,
        taker_contact: 'taker_contact',
      })
    }).rejects.toThrow()
    expect(tradeId).toBe(0)
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

describe('trade limits', () => {
  let offer: GetOffer
  it('should have an available offer', async () => {
    offer = (await getOrCreateOffer(makerClient)).offer
    expect(offer).toBeDefined()
  })
  it('should not allow a trade to have an amount bellow the trade limit min', async () => {
    offer = (await getOrCreateOffer(makerClient)).offer
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    // Try to create a trade with the amount bellow the limit
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encryption_key = takerSecrets.publicKey
    let tradeId = 0
    // Query Price for Offer denom
    const usdPrice = await takerClient.updateFiatPrice(FiatCurrency.USD, offer.denom)
    const fiatPriceDecimals = 100
    const price = usdPrice.price * (parseInt(offer.rate) / fiatPriceDecimals)
    const denomDecimals = 1_000_000
    const invalidAmount = (hubInfo.hubConfig.trade_limit_min / price) * denomDecimals * fiatPriceDecimals * 0.96 // 4% bellow the limit min

    await expect(async () => {
      tradeId = await takerClient.openTrade({
        amount: invalidAmount.toFixed(0),
        offer_id: offer.id,
        taker: takerClient.getWalletAddress(),
        profile_taker_contact,
        profile_taker_encryption_key,
        taker_contact: 'taker_contact',
      })
    }).rejects.toThrow(/Invalid trade amount/)
    expect(tradeId).toBe(0)
  })
  it('should not allow a trade to have an amount above the trade limit max', async () => {
    offer = (await getOrCreateOffer(makerClient)).offer
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    // Try to create a trade with the amount above the limit
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    let tradeId = 0
    // Query Price for Offer denom
    const usdPrice = await takerClient.updateFiatPrice(FiatCurrency.USD, offer.denom)
    console.log('usd price for denom', usdPrice.price, offer.denom)
    const fiatPriceDecimals = 100
    const price = usdPrice.price * (parseInt(offer.rate) / fiatPriceDecimals)
    const denomDecimals = 1_000_000
    const invalidAmount = (hubInfo.hubConfig.trade_limit_max / price) * denomDecimals * fiatPriceDecimals * 1.04 // 4% above the limit
    console.log('invalidAmount', invalidAmount)

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
    expect(tradeId).toBe(0)
  })
  it('should not allow a trader to have more active trades than the hub limit', async () => {
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    const hubCfg = hubInfo.hubConfig
    // Fetch taker profile
    const takerProfile = await takerClient.fetchProfile()
    // Update Hub Config to have a limit of profile active trades + 1
    hubCfg.active_trades_limit = takerProfile.active_trades_count + 1
    const updateHubMsg = createHubUpdateConfigMsg(
      hubCfg.offer_addr,
      hubCfg.trade_addr,
      hubCfg.price_addr,
      hubCfg.profile_addr
    )
    const newTradesLimit = (updateHubMsg.update_config.active_trades_limit = takerProfile.active_trades_count + 1)
    await adminClient
      .getCwClient()
      .execute(adminClient.getWalletAddress(), hubInfo.hubAddress, updateHubMsg, 'auto', 'update hub config')
    // Check that the updated hubInfo has the new limit
    await takerClient.updateHub(hubInfo.hubAddress)
    expect(takerClient.getHubInfo().hubConfig.active_trades_limit).toBe(newTradesLimit)

    // Fetch Offer and USD price for denom
    const offer = (await getOrCreateOffer(makerClient)).offer
    const tradeAmount = await getValidTradeAmount(takerClient, offer)

    // Create a trade and expect it to be successful
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    const tradeId = await takerClient.openTrade({
      amount: tradeAmount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key: profile_taker_encrypt_key,
      taker_contact: 'taker_contact',
    })
    expect(tradeId).not.toBeNaN()
    // Accept the trade to start to count as active trade
    if (offer.offer_type === OfferType.sell) {
      const trade = await makerClient.fetchTradeDetail(tradeId)
      await makerClient.fundEscrow(trade, 'maker_contract')
    } else {
      await makerClient.acceptTradeRequest(tradeId, 'maker_contact')
    }

    // Try to create another trade and expect it to fail
    await expect(async () => {
      await takerClient.openTrade({
        amount: tradeAmount,
        offer_id: offer.id,
        taker: takerClient.getWalletAddress(),
        profile_taker_contact,
        profile_taker_encryption_key: profile_taker_encrypt_key,
        taker_contact: 'taker_contact',
      })
    }).rejects.toThrow(/Active trades limit reached/)
  })
  it('should not allow to create more offers than the hub limit', async () => {
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    const hubCfg = hubInfo.hubConfig
    // Fetch maker profile
    const makerProfile = await makerClient.fetchProfile()
    console.log('makerProfile', makerProfile)
    // Update Hub Config to have a limit of profile active offers + 1
    const updateHubMsg = createHubUpdateConfigMsg(
      hubCfg.offer_addr,
      hubCfg.trade_addr,
      hubCfg.price_addr,
      hubCfg.profile_addr
    )
    const newOffersLimit = (updateHubMsg.update_config.active_offers_limit = makerProfile.active_offers_count + 1)
    console.log('newOffersLimit', newOffersLimit)
    await adminClient
      .getCwClient()
      .execute(adminClient.getWalletAddress(), hubInfo.hubAddress, updateHubMsg, 'auto', 'update hub config')
    // Check that the updated hubInfo has the new limit
    await makerClient.updateHub(hubInfo.hubAddress)
    expect(makerClient.getHubInfo().hubConfig.active_offers_limit).toBe(newOffersLimit)
    // Create an offer and expect it to be successful
    const offer = await getOrCreateOffer(makerClient, true)
    expect(offer).toBeDefined()
    // Fetch updated maker profile
    const updatedMakerProfile = await makerClient.fetchProfile()
    console.log('updatedMakerProfile', updatedMakerProfile)
    // Try to create another offer and expect it to fail
    await expect(async () => {
      await makerClient.createOffer(offers[0] as PostOffer)
    }).rejects.toThrow(/Active offers limit reached/)
  })
  it('should decrease the number of active offers when pausing or archiving an offer', async () => {
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    const hubCfg = hubInfo.hubConfig
    // Fetch maker profile
    const makerProfile = await makerClient.fetchProfile()
    const makerActiveOffers = makerProfile.active_offers_count
    // Update Hub Config to have a limit of profile active offers + 1
    const updateHubMsg = createHubUpdateConfigMsg(
      hubCfg.offer_addr,
      hubCfg.trade_addr,
      hubCfg.price_addr,
      hubCfg.profile_addr
    )
    const newOffersLimit = (updateHubMsg.update_config.active_offers_limit = makerProfile.active_offers_count + 1)
    await adminClient
      .getCwClient()
      .execute(adminClient.getWalletAddress(), hubInfo.hubAddress, updateHubMsg, 'auto', 'update hub config')
    // Check that the updated hubInfo has the new limit
    await makerClient.updateHub(hubInfo.hubAddress)
    expect(makerClient.getHubInfo().hubConfig.active_offers_limit).toBe(newOffersLimit)
    // Create an offer and expect it to be successful
    const offerResponse = await getOrCreateOffer(makerClient, true)
    expect(offerResponse).toBeDefined()
    // Pause the offer and expect the active offers count to be decreased
    await makerClient.updateOffer({ ...offerResponse.offer, state: OfferState.paused })
    const updatedMakerProfile = await makerClient.fetchProfile()
    expect(updatedMakerProfile.active_offers_count).toBe(makerActiveOffers)
    // Activate the offer and expect the active offers count to be increased
    await makerClient.updateOffer({ ...offerResponse.offer, state: OfferState.active })
    const updatedMakerProfile2 = await makerClient.fetchProfile()
    expect(updatedMakerProfile2.active_offers_count).toBe(makerActiveOffers + 1)
    // Archive the offer and expect the active offers count to be decreased
    await makerClient.updateOffer({ ...offerResponse.offer, state: OfferState.archived })
    const updatedMakerProfile3 = await makerClient.fetchProfile()
    expect(updatedMakerProfile3.active_offers_count).toBe(makerActiveOffers)
  })
  it('it should decrease the number of active trades when canceling a trade request', async () => {
    // Get Hub Info
    const hubInfo = takerClient.getHubInfo()
    const hubCfg = hubInfo.hubConfig
    // Fetch taker profile
    const takerProfile = await takerClient.fetchProfile()
    const takerActiveTrades = takerProfile.active_trades_count
    // Update Hub Config to have a limit of profile active trades + 1
    const updateHubMsg = createHubUpdateConfigMsg(
      hubCfg.offer_addr,
      hubCfg.trade_addr,
      hubCfg.price_addr,
      hubCfg.profile_addr
    )
    const newTradesLimit = (updateHubMsg.update_config.active_trades_limit = takerProfile.active_trades_count + 1)
    await adminClient
      .getCwClient()
      .execute(adminClient.getWalletAddress(), hubInfo.hubAddress, updateHubMsg, 'auto', 'update hub config')
    // Check that the updated hubInfo has the new limit
    await takerClient.updateHub(hubInfo.hubAddress)
    expect(takerClient.getHubInfo().hubConfig.active_trades_limit).toBe(newTradesLimit)

    // Fetch Offer and USD price for denom
    const offer = (await getOrCreateOffer(makerClient)).offer
    const tradeAmount = await getValidTradeAmount(takerClient, offer)

    // Create a trade and expect it to be successful
    const profile_taker_contact = await encryptDataMocked(takerSecrets.publicKey, takerContact)
    const profile_taker_encrypt_key = takerSecrets.publicKey
    const tradeId = await takerClient.openTrade({
      amount: tradeAmount,
      offer_id: offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact,
      profile_taker_encryption_key: profile_taker_encrypt_key,
      taker_contact: 'taker_contact',
    })
    expect(tradeId).not.toBeNaN()
    // Cancel the trade and expect the active trades count to be decreased
    await takerClient.cancelTradeRequest(tradeId)
    const updatedTakerProfile = await takerClient.fetchProfile()
    expect(updatedTakerProfile.active_trades_count).toBe(takerActiveTrades)
  })
  it('should send fee amount on fund escrow if maker is selling', async () => {
    // Get Hub Info
    const hubInfo = makerClient.getHubInfo()
    const hubConfig = hubInfo.hubConfig
    // Fetch maker profile
    const makerProfile = await makerClient.fetchProfile()
    // If maker has same active offers as Hub Limit, increase it by 1
    if (makerProfile.active_offers_count === hubConfig.active_offers_limit) {
      const updateHubMsg = createHubUpdateConfigMsg(
        hubConfig.offer_addr,
        hubConfig.trade_addr,
        hubConfig.price_addr,
        hubConfig.profile_addr
      )
      const newOffersLimit = (updateHubMsg.update_config.active_offers_limit = makerProfile.active_offers_count + 1)
      const newActiveTradesLimit = (updateHubMsg.update_config.active_trades_limit =
        makerProfile.active_trades_count + 1)
      await adminClient
        .getCwClient()
        .execute(adminClient.getWalletAddress(), hubInfo.hubAddress, updateHubMsg, 'auto', 'update hub config')
      // Check that the updated hubInfo has the new limit
      await makerClient.updateHub(hubInfo.hubAddress)
      expect(makerClient.getHubInfo().hubConfig.active_offers_limit).toBe(newOffersLimit)
      expect(makerClient.getHubInfo().hubConfig.active_trades_limit).toBe(newActiveTradesLimit)
    }
    // Create an Offer of type sell
    const offerResponse = await getOrCreateOffer(makerClient, true, OfferType.sell)
    expect(offerResponse).toBeDefined()
    // Fetch Offer and USD price for denom
    const offer = offerResponse.offer
    const createTradeAmount = await getValidTradeAmount(takerClient, offer)
    // Query the balance of the Denom of the trade owned by the trade contract
    const tradeBalance = await makerClient
      .getCwClient()
      .getBalance(hubInfo.hubConfig.trade_addr, denomToValue(offerResponse.offer.denom))
    // Create a trade
    const tradeId = await takerClient.openTrade({
      amount: createTradeAmount,
      offer_id: offerResponse.offer.id,
      taker: takerClient.getWalletAddress(),
      profile_taker_contact: 'profile_taker_contact',
      profile_taker_encryption_key: 'profile_taker_encryption_key',
      taker_contact: 'taker_contact',
    })
    expect(tradeId).not.toBeNaN()
    const tradeInfo = (await takerClient.fetchTradeDetail(tradeId)) as TradeInfo
    // Maker funds the escrow
    const encryptedMakerContact = await encryptDataMocked(makerSecrets.publicKey, makerContact)
    await makerClient.fundEscrow(tradeInfo, encryptedMakerContact)
    // Query the Updated Trade Contract balance
    const tradeBalanceAfter = await makerClient
      .getCwClient()
      .getBalance(hubInfo.hubConfig.trade_addr, denomToValue(offerResponse.offer.denom))
    // Calculate the difference between the balances
    console.log('tradeBalance', tradeBalance)
    console.log('tradeBalanceAfter', tradeBalanceAfter)
    const balanceIncrease = Math.floor(Number(tradeBalanceAfter.amount) - Number(tradeBalance.amount))
    const totalFeePct =
      Number(hubConfig.burn_fee_pct) + Number(hubConfig.chain_fee_pct) + Number(hubConfig.warchest_fee_pct)
    console.log('totalFeePct', totalFeePct)
    const tradeAmount = Math.floor(Number(tradeInfo.trade.amount))
    const feeAmount = tradeAmount * totalFeePct
    console.log('feeAmount', feeAmount)
    console.log('tradeAmount', tradeAmount)
    console.log('balanceIncrease', balanceIncrease)
    expect(balanceIncrease).toBe(Math.floor(tradeAmount + feeAmount))
    // Taker should set the trade state to fiat_deposited
    await takerClient.setFiatDeposited(tradeId)
    // Maker should release the escrow
    await makerClient.releaseEscrow(tradeId)
    // CHeck that trade state is escrow_released
    const tradeInfoAfter = (await takerClient.fetchTradeDetail(tradeId)) as TradeInfo
    expect(tradeInfoAfter.trade.state).toBe(TradeState.escrow_released)
    // Archive Offer
    await makerClient.updateOffer({ ...offerResponse.offer, state: OfferState.archived })
  })
})
