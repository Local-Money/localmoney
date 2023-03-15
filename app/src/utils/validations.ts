import type { GetOffer, Trade } from '~/types/components.interface'
import { checkMicroDenomAvailable, denomToValue } from '~/utils/denom'
import { checkFiatAvailable } from '~/utils/fiat'
import { TradeState } from '~/types/components.interface'
import { ChainClient } from '~/network/Chain'

export function checkValidOffer(offer: GetOffer, chainClient: ChainClient = ChainClient.kujiraMainnet): boolean {
  return checkMicroDenomAvailable(denomToValue(offer.denom), chainClient) && checkFiatAvailable(offer.fiat_currency)
}

export function checkTradeNeedsRefund(trade: Trade, userAddr: string): boolean {
  const isSeller = trade.seller === userAddr
  const lastStateIndex = trade.state_history.length - 1
  const lastTradeState = trade.state_history[lastStateIndex].state
  return (
    isSeller &&
    ((trade.state === TradeState.request_expired && lastTradeState === TradeState.escrow_funded) ||
      trade.state === TradeState.escrow_canceled)
  )
}
