import fs from 'fs'
import type { InstantiateResult, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { TestCosmosChain } from './network/TestCosmosChain'
import codeIds from './fixtures/codeIds.json'
import { TRADE_DISPUTE_TIMER, TRADE_EXPIRATION_TIMER } from './configs'
import { encryptDataMocked } from './helper'
import makerSecrets from './fixtures/maker_secrets.json'
import offers from './fixtures/offers.json'
import { DEV_CONFIG, DEV_HUB_INFO } from '~/network/cosmos/config/dev'
import type { OfferResponse, PostOffer } from '~/types/components.interface'
import { OfferType } from '~/types/components.interface'

export function createHubUpdateConfigMsg(offerAddr: string, tradeAddr: string, priceAddr: string, profileAddr: string) {
  return {
    update_config: {
      offer_addr: offerAddr,
      trade_addr: tradeAddr,
      price_addr: priceAddr,
      profile_addr: profileAddr,
      price_provider_addr: process.env.PRICE_PROVIDER_ADDR,
      local_market_addr: process.env.LOCAL_MARKET,
      local_denom: { native: process.env.LOCAL_DENOM },
      chain_fee_collector_addr: process.env.CHAIN_FEE_COLLECTOR,
      warchest_addr: process.env.WARCHEST_ADDR,
      trade_limit_min: '1', // in USD
      trade_limit_max: '100', // in USD
      active_offers_limit: 3,
      active_trades_limit: 10,
      arbitration_fee_pct: '0.01', // 1%
      burn_fee_pct: '0.002', // 0.2%
      chain_fee_pct: '0.003', // 0.3%
      warchest_fee_pct: '0.005', // 0.5%
      trade_expiration_timer: TRADE_EXPIRATION_TIMER,
      trade_dispute_timer: TRADE_DISPUTE_TIMER,
    },
  }
}

/**
 * Instantiate all contracts and setup Hub:
 */
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

  const priceProviderClient = new TestCosmosChain(DEV_CONFIG, DEV_HUB_INFO)
  priceProviderClient.seed = process.env.PRICE_PROVIDER_SEED!
  await priceProviderClient.connectWallet()

  if (process.env.HUB) {
    await adminClient.updateHub(process.env.HUB)
    await makerClient.updateHub(process.env.HUB)
    await takerClient.updateHub(process.env.HUB)
    await priceProviderClient.updateHub(process.env.HUB)
  } else {
    // Instantiate all contracts
    const admAddr = adminClient.getWalletAddress()
    const adminCwClient = adminClient.getCwClient() as SigningCosmWasmClient

    const instantiateMsg = { admin_addr: admAddr }
    const { hub, offer, trade, price, profile } = codeIds
    const opts = { admin: admAddr }
    const hubInstantiateResult = await adminCwClient.instantiate(admAddr, hub, instantiateMsg, 'hub', 'auto', opts)
    console.log('Hub Instantiate Result: ', hubInstantiateResult)
    const offerInstantiateResult = await adminCwClient.instantiate(
      admAddr,
      offer,
      instantiateMsg,
      'offer',
      'auto',
      opts
    )
    console.log('Offer Instantiate Result: ', offerInstantiateResult)
    const tradeInstantiateResult = await adminCwClient.instantiate(
      admAddr,
      trade,
      instantiateMsg,
      'trade',
      'auto',
      opts
    )
    console.log('Trade Instantiate Result: ', tradeInstantiateResult)
    const profileResult = await adminCwClient.instantiate(admAddr, profile, instantiateMsg, 'profile', 'auto', opts)
    console.log('Profile Instantiate Result: ', profileResult)

    // To run to test suit in the testnet
    let priceResult: InstantiateResult
    if (process.env.PRICE_ADDR) {
      priceResult = { contractAddress: process.env.PRICE_ADDR as string } as InstantiateResult
    } else {
      priceResult = await adminCwClient.instantiate(admAddr, price, instantiateMsg, 'price', 'auto', opts)
    }
    console.log('Price Instantiate Result: ', priceResult)

    // Assert that all contracts were instantiated
    const results = [hubInstantiateResult, offerInstantiateResult, tradeInstantiateResult, priceResult, profileResult]
    results.forEach((result: InstantiateResult) => {
      expect(result).toHaveProperty('contractAddress')
    })

    // Setup Hub
    const updatedConfigMsg = createHubUpdateConfigMsg(
      offerInstantiateResult.contractAddress,
      tradeInstantiateResult.contractAddress,
      priceResult.contractAddress,
      profileResult.contractAddress
    )
    const updateCfgResult = await adminCwClient.execute(
      admAddr,
      hubInstantiateResult.contractAddress,
      updatedConfigMsg,
      'auto'
    )
    console.log('Update Config Result: ', updateCfgResult)
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
    console.log('Prive Provider Wallet: ', priceProviderClient.getWalletAddress())
    console.log('Hub Address:', hubInstantiateResult.contractAddress)
    fs.appendFileSync('.env', `\r\nHUB=${hubInstantiateResult.contractAddress}`)
  }
  return { adminClient, makerClient, takerClient, priceProviderClient }
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

/**
 * Creates an offer if the maker doesn't have any.
 * @param makerClient the maker client.
 * @param forceCreate if true, it will create a new offer even if the maker has one.
 * @returns an offer.
 */
export async function getOrCreateOffer(
  makerClient: TestCosmosChain,
  forceCreate = false,
  offer_type = OfferType.buy
): Promise<OfferResponse> {
  let myOffers = await makerClient.fetchMyOffers()
  if (myOffers.length === 0 || forceCreate) {
    const makerContact = 'maker001'
    const owner_contact = await encryptDataMocked(makerSecrets.publicKey, makerContact)
    const owner_encryption_key = makerSecrets.publicKey
    const denom = { native: process.env.OFFER_DENOM! }
    const newOffer = { ...offers[0], owner_contact, owner_encryption_key, denom, offer_type } as PostOffer
    await makerClient.createOffer(newOffer)
  }
  myOffers = await makerClient.fetchMyOffers()
  return myOffers[0]
}
