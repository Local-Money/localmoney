import { TextDecoder, TextEncoder } from 'util'
import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import prices from './fixtures/update_prices.json'
import register_price_route_for_denom from './fixtures/register_price_route_for_denom.json'
import { FiatCurrency } from '~/types/components.interface'
import 'isomorphic-fetch'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let takerClient: TestCosmosChain
let adminClient: TestCosmosChain

jest.setTimeout(60 * 1000)
let priceAddr = ''
beforeAll(async () => {
  const result = await setupProtocol()
  takerClient = result.takerClient
  adminClient = result.adminClient
  priceAddr = takerClient.getHubInfo().hubConfig.price_addr
})

describe('price tests', () => {
  it('should register fiat prices', async () => {
    const result = await takerClient
      .getCwClient()
      .execute(takerClient.getWalletAddress(), priceAddr, prices, 'auto', 'register fiat prices')
    expect(result.transactionHash).not.toBeNull()
  })
  it('should register price route', async () => {
    const result = await adminClient
      .getCwClient()
      .execute(
        adminClient.getWalletAddress(),
        priceAddr,
        register_price_route_for_denom,
        'auto',
        'register price route'
      )
    expect(result.transactionHash).not.toBeNull()
  })
  it('should query fiat prices for denom', async () => {
    const arsPrice = await takerClient.fetchFiatPriceForDenom(FiatCurrency.ARS, { native: process.env.OFFER_DENOM! })
    const brlPrice = await takerClient.fetchFiatPriceForDenom(FiatCurrency.BRL, { native: process.env.OFFER_DENOM! })
    const copPrice = await takerClient.fetchFiatPriceForDenom(FiatCurrency.COP, { native: process.env.OFFER_DENOM! })
    expect(arsPrice.price * 1).toBeGreaterThan(0)
    expect(brlPrice.price * 1).toBeGreaterThan(0)
    expect(copPrice.price * 1).toBeGreaterThan(0)
  })
})
