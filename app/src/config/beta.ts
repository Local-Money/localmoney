import { ChainClient } from '~/network/Chain'
import type { Addr } from '~/types/components.interface'
import makers from '~/config/makers.json'

export function isBetaMaker(addr: Addr, chainClient: ChainClient): boolean {
  // TODO Changes the validation to validate if it is the Mainnent (pending of the LOCAL-1004)
  return chainClient === ChainClient.dev ? makers.includes(addr) : true
}
