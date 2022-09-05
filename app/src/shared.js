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

export function formatFiatAmount(amount) {
  return amount.toFixed(2)
}

export function getTradeCounterParty(walletAddress, trade) {
  return walletAddress === trade.seller ? trade.buyer : trade.seller
}

export function timeSince(date) {
  const seconds = Math.floor((new Date() - date) / 1000)

  let interval = seconds / 31536000

  if (interval > 1) {
    return `${Math.floor(interval)}y ago`
  }
  interval = seconds / 2592000
  if (interval > 1) {
    return `${Math.floor(interval)}m ago`
  }
  interval = seconds / 86400
  if (interval > 1) {
    return `${Math.floor(interval)}d ago`
  }
  interval = seconds / 3600
  if (interval > 1) {
    return `${Math.floor(interval)}h ago`
  }
  interval = seconds / 60
  if (interval > 1) {
    return `${Math.floor(interval)}m ago`
  }
  return `${Math.floor(seconds)}s ago`
}

export function formatTradeState(state) {
  return {
    request_created: 'Expired',
    request_accepted: 'Expired',
    escrow_funded: 'Expired',
    request_expired: 'Expired',
    request_canceled: 'Canceled',
    escrow_refunded: 'Refunded',
    escrow_released: 'Completed',
    settled_for_maker: 'settled_for_maker',
    settled_for_taker: 'settled_for_taker',
  }[state]
}

export function calculateFiatPriceByRate(usdRate, offerRate) {
  if (offerRate === 0) {
    return usdRate
  }

  return usdRate * (offerRate / 100)
}

export function convertOfferRateToMarginRate(offerRate) {
  if (offerRate > 100) {
    return {
      marginOffset: offerRate - 100,
      margin: 'above',
    }
  } else {
    return {
      marginOffset: 100 - offerRate,
      margin: 'below',
    }
  }
}

export function convertMarginRateToOfferRate(margin, marginOffset) {
  return 100 + (margin === 'above' ? +marginOffset : -marginOffset)
}

export function formatDate(date, showTime = true) {
  const year = new Intl.DateTimeFormat('en', { year: 'numeric' }).format(date)
  const month = new Intl.DateTimeFormat('en', { month: 'short' }).format(date)
  const day = new Intl.DateTimeFormat('en', { day: '2-digit' }).format(date)
  if (showTime) {
    const time = new Intl.DateTimeFormat('en', {
      hour: '2-digit',
      minute: '2-digit',
    }).format(date)
    return `${day} ${month} ${year} - ${time}`
  } else {
    return `${day} ${month} ${year}`
  }
}

/** UI Elements **/
export function scrollToElement(el) {
  if (el) {
    el.scrollIntoView({ behavior: 'smooth', block: 'center' })
  }
}

/** Trade State **/
export function tradeCanBeFunded(tradeInfo, walletAddr) {
  const { trade } = tradeInfo
  return trade.state === 'created' && trade.seller === walletAddr
}

export function tradeCanBeReleased(tradeInfo, walletAddr) {
  const { trade } = tradeInfo
  return trade.state === 'escrow_funded' && trade.seller === walletAddr
}

export function tradeCanBeRefunded(tradeInfo, walletAddr) {
  const { trade } = tradeInfo
  return trade.state === 'escrow_funded' && tradeInfo.expired && trade.sender === walletAddr
}
