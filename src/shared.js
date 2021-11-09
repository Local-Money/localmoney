/** String Formatting **/
export function formatAddress(address) {
  const start = address.substr(0, 8)
  const length = address.length
  const end = address.substr(length - 6, length - 1)
  return `${start}...${end}`
}

export function formatAmount(amount, ustAmount = true) {
  if (ustAmount) {
    amount = amount / 1000000
  }
  return amount.toFixed(2)
}

export function formatTradeState(state) {
  return {
    'created': 'Created',
    'escrow_funded': 'Escrow Funded',
    'closed': 'Closed',
    'canceled': 'Canceled'
  }[state]
}

/** UI Elements **/
export function scrollToElement(el) {
  if (el) {
    el.scrollIntoView({behavior: 'smooth'})
  }
}

/** Trade State **/
export function tradeCanBeFunded(tradeInfo, walletAddr) {
  const { trade, offer } = tradeInfo;
  return trade.state === 'created'
      && offer.owner === walletAddr
      && offer.offer_type === 'sell';
}

export function tradeCanBeReleased(tradeInfo, walletAddr) {
  const { trade, offer } = tradeInfo;
  return trade.state === 'escrow_funded'
      && offer.owner === walletAddr
      && offer.offer_type === 'sell';
}
export function tradeCanBeRefunded(tradeInfo, walletAddr, currentHeight) {
  const { trade, offer } = tradeInfo;
  return trade.state === 'escrow_funded'
      && trade.expire_height >= currentHeight
      && offer.owner === walletAddr
      && offer.offer_type === 'sell';
}