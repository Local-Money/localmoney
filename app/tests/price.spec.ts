import { TextDecoder, TextEncoder } from 'util'
import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import prices from './fixtures/update_prices.json'
import priceRoutes from './fixtures/register_price_route_for_denom.json'
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
  it('should register price routes', async () => {
    // Iterate over all the price routes and register them
    for (const i in priceRoutes) {
      const msg = priceRoutes[i]
      const result = await adminClient
        .getCwClient()
        .execute(adminClient.getWalletAddress(), priceAddr, msg, 'auto', 'register price route')
      expect(result.transactionHash).not.toBeNull()
    }
  })
  it('should query fiat prices for denoms', async () => {
    // Iterate over all the price routes and fetchFiatPriceForDenom for them for ARS, BRL and COP
    // expect all prices * 1 to be greater than 0
    for (const i in priceRoutes) {
      const priceRoute = priceRoutes[i]
      const arsPrice = await takerClient.fetchFiatPriceForDenom(
        FiatCurrency.ARS,
        priceRoute.register_price_route_for_denom.denom
      )
      const brlPrice = await takerClient.fetchFiatPriceForDenom(
        FiatCurrency.BRL,
        priceRoute.register_price_route_for_denom.denom
      )
      const copPrice = await takerClient.fetchFiatPriceForDenom(
        FiatCurrency.COP,
        priceRoute.register_price_route_for_denom.denom
      )
      expect(arsPrice.price * 1).toBeGreaterThan(0)
      expect(brlPrice.price * 1).toBeGreaterThan(0)
      expect(copPrice.price * 1).toBeGreaterThan(0)
    }
  })
})
