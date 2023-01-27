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
  it.only('should register conversion route for USK', async () => {
    const usk = { native: 'factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk' }
    const local = { native: 'factory/kujira1swkuyt08z74n5jl7zr6hx0ru5sa2yev5v896p6/local' }
    await adminClient.getCwClient().execute(
      adminClient.getWalletAddress(),
      tradeAddr,
      {
        register_conversion_route_for_denom: {
          denom: usk,
          route: [
            {
              ask_asset: local,
              offer_asset: usk,
              pool: 'kujira1sse6a00arh9dalzsyrd3q825dsn2zmrag0u4qx8q0dyks4ftnxyqrj0xds',
            },
          ],
        },
      },
      'auto'
    )
  })
})
