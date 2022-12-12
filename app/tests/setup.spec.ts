import { TextDecoder, TextEncoder } from 'util'
import { jest } from '@jest/globals'
import dotenv from 'dotenv'
import { setupProtocol } from './utils'
import type { TestCosmosChain } from './network/TestCosmosChain'

dotenv.config()
Object.assign(global, { TextEncoder, TextDecoder })

let takerClient: TestCosmosChain
let adminClient: TestCosmosChain

jest.setTimeout(60 * 1000)
beforeAll(async () => {
  const result = await setupProtocol()
  takerClient = result.takerClient
  adminClient = result.adminClient
})

describe('setup protocol', () => {
  it('query hub config', async () => {
    expect(adminClient.getHubInfo().hubConfig).toBeDefined()
    expect(takerClient.getHubInfo().hubConfig).toEqual(adminClient.getHubInfo().hubConfig)
  })
})
