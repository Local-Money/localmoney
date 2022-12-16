import { TextDecoder, TextEncoder } from 'util'
import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import makerSecrets from './fixtures/maker_secrets.json'
import { FiatCurrency, OfferOrder, OfferType } from '~/types/components.interface'
import 'isomorphic-fetch'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

const args = {
  denom: { native: 'ukuji' },
  fiatCurrency: FiatCurrency.ARS,
  offerType: OfferType.sell,
  order: OfferOrder.trades_count,
}
const limit = 3

let takerClient: TestCosmosChain
let makerClient: TestCosmosChain
jest.setTimeout(60 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  takerClient = result.takerClient
  makerClient = result.makerClient
})

describe('offers pagination', () => {
  it(`maker must have at least ${limit} offers`, async () => {
    let myOffers = await makerClient.fetchMyOffers(limit)
    if (myOffers.length < limit) {
      for (let i = 0; i < limit - myOffers.length; i++) {
        await makerClient.createOffer({
          denom: args.denom,
          fiat_currency: args.fiatCurrency,
          min_amount: '100000',
          max_amount: '1000000',
          offer_type: args.offerType,
          owner_contact: 'maker001',
          owner_encryption_key: makerSecrets.publicKey,
          rate: (100 - i).toString(),
        })
      }
      myOffers = await makerClient.fetchMyOffers(limit)
    }
    expect(myOffers.length).toBe(limit)
  })
  it('maker should be able to paginate the list of offers', async () => {
    let myOffers = await makerClient.fetchMyOffers(limit)
    for (let i = 0; i < myOffers.length - 1; i++) {
      const previousId = myOffers[myOffers.length - 2].offer.id
      const last = myOffers[myOffers.length - 1].offer.id
      myOffers = await makerClient.fetchMyOffers(myOffers.length, last)
      expect(myOffers[myOffers.length - 1].offer.id).toBe(previousId)
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
