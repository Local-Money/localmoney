import { ChainClient } from '../src/network/Chain'
import { useClientStore } from '../src/stores/client'

const client = useClientStore()
await client.setClient(ChainClient.testCosmos) // required to properly init chain

// eslint-disable-next-line no-undef
describe('Trade Lifecycle Happy Path', () => {

})
