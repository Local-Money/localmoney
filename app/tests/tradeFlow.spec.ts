import { TextDecoder, TextEncoder } from 'util'

import type { Chain } from '~/network/Chain'
import { ChainClient, chainFactory } from '~/network/Chain'
global.TextEncoder = TextEncoder
global.TextDecoder = TextDecoder

let client: Chain

describe('Trade Lifecycle Happy Path', () => {
  beforeAll(async () => {
    client = chainFactory(ChainClient.testCosmos)
    await client.init()
    await client.connectWallet()
  })
  it('Wallet address length should be greater than zero.', async () => {
    expect(client.getWalletAddress().length).toBeGreaterThan(0)

  })
  it('should fetch offers', async () => {
    const offers = await client.fetchMyOffers()
    expect(offers.length).toBeGreaterThan(0)
  })
})
