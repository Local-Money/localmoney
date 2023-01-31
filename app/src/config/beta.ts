import { ChainClient } from '~/network/Chain'
import type { Addr } from '~/types/components.interface'

export async function isBetaMaker(addr: Addr, chainClient: ChainClient): Promise<boolean> {
  // TODO Changes the validation to validate if it is the Mainnent (pending of the LOCAL-1004)
  const makers = await fetch('data/makers.json').then((res) => res.json())
  return chainClient === ChainClient.kujiraMainnet ? makers.includes(addr) : true
}
