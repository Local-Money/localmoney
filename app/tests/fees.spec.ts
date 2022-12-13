import { TextDecoder, TextEncoder } from 'util'
import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'
import 'isomorphic-fetch'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let takerClient: TestCosmosChain
let adminClient: TestCosmosChain

jest.setTimeout(60 * 1000)
let tradeAddr = ''
beforeAll(async () => {
  const result = await setupProtocol()
  takerClient = result.takerClient
  adminClient = result.adminClient
  tradeAddr = takerClient.getHubInfo().hubConfig.trade_addr
})

describe('fees tests', () => {
  it('should register conversion route for KUJI', async () => {
    await adminClient.getCwClient().execute(
      adminClient.getWalletAddress(),
      tradeAddr,
      {
        register_conversion_route_for_denom: {
          denom: { native: 'ukuji' },
          route: [
            {
              ask_asset: {
                native: process.env.LOCAL_DENOM,
              },
              offer_asset: { native: 'ukuji' },
              pool: process.env.LOCAL_MARKET,
            },
          ],
        },
      },
      'auto'
    )
  })
  it('should register conversion route for USK', async () => {
    const usk = { native: 'factory/kujira1r85reqy6h0lu02vyz0hnzhv5whsns55gdt4w0d7ft87utzk7u0wqr4ssll/uusk' }
    const ukuji = { native: 'ukuji' }
    await adminClient.getCwClient().execute(
      adminClient.getWalletAddress(),
      tradeAddr,
      {
        register_conversion_route_for_denom: {
          denom: usk,
          route: [
            {
              ask_asset: ukuji,
              offer_asset: usk,
              pool: 'kujira1wl003xxwqltxpg5pkre0rl605e406ktmq5gnv0ngyjamq69mc2kqm06ey6',
            },
            {
              ask_asset: {
                native: process.env.LOCAL_DENOM,
              },
              offer_asset: ukuji,
              pool: process.env.LOCAL_MARKET,
            },
          ],
        },
      },
      'auto'
    )
  })
})
