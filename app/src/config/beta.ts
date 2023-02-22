import { ChainClient } from '~/network/Chain'
import type { Addr } from '~/types/components.interface'

export function isBetaMaker(addr: Addr, chainClient: ChainClient, betaMakers: String[]): boolean {
  return chainClient === ChainClient.kujiraMainnet ? betaMakers.includes(addr) : true
}
