import type { Addr, Arbitrator } from '~/types/components.interface'

export function enableMyOffers(userWallet: { isConnected: boolean; address: Addr }): boolean {
  return userWallet.isConnected
}

export function enableDisputes(
  userWallet: { isConnected: boolean; address: Addr },
  arbitrators: Arbitrator[]
): boolean {
  return userWallet.isConnected && arbitrators.filter((a) => a.arbitrator === userWallet.address).length > 0
}
