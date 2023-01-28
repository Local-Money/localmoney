import type { Addr, Arbitrator } from '~/types/components.interface'
import { isBetaMaker } from '~/config/beta'
import type { ChainClient } from '~/network/Chain'

export function enableMyOffers(userWallet: { isConnected: boolean; address: Addr }, chainClient: ChainClient): boolean {
  return userWallet.isConnected && isBetaMaker(userWallet.address, chainClient)
}
export function enableDisputes(
  userWallet: { isConnected: boolean; address: Addr },
  arbitrators: Arbitrator[]
): boolean {
  return userWallet.isConnected && arbitrators.filter((a) => a.arbitrator === userWallet.address).length > 0
}
