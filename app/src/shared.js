import { decryptData } from '~/utils/crypto'

/** String Formatting **/
export function formatAddress(address) {
  const start = address.substr(0, 8)
  const length = address.length
  const end = address.substr(length - 6, length - 1)
  return `${start}...${end}`
}

export function formatAmount(amount, isCrypto = true, decimals = 2) {
  if (isCrypto) {
    amount = amount / 1000000
  }
  return parseFloat(amount.toFixed(decimals))
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
    escrow_refunded: 'Canceled',
    escrow_released: 'Completed',
    settled_for_maker: 'Settled',
    settled_for_taker: 'Settled',
  }[state]
}

export function calculateFiatPriceByRate(fiatPrice, offerRate) {
  if (offerRate === 0) {
    return fiatPrice
  }

  return fiatPrice * (offerRate / 100)
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

export function formatDate(date, showTime = true, showDay = true) {
  const year = new Intl.DateTimeFormat('en', { year: 'numeric' }).format(date)
  const month = new Intl.DateTimeFormat('en', { month: 'short' }).format(date)
  const day = new Intl.DateTimeFormat('en', { day: '2-digit' }).format(date)
  let result = ''
  if (showDay) {
    result += `${day} `
  }
  result += `${month} ${year}`
  if (showTime) {
    const time = new Intl.DateTimeFormat('en', {
      hour: '2-digit',
      minute: '2-digit',
    }).format(date)
    result += ` - ${time}`
  }
  return result
}

/**
 * The Telegram handle needs to have at least 5 characters
 *
 * @param telegram
 * @returns {boolean}
 */
export function isTelegramHandleValid(telegram) {
  const handle = removeTelegramHandlePrefix(telegram)
  return handle.length >= 5
}

export function addTelegramHandlePrefix(telegram) {
  return `@${telegram}`
}

export function removeTelegramHandlePrefix(telegram) {
  const search = ['t.me/', '@']
  const index = search.findIndex((s) => telegram.includes(s))
  if (index === -1) {
    return telegram
  }
  const start = telegram.indexOf(search[index]) + search[index].length
  return telegram.includes(search[index]) ? telegram.substring(start) : telegram
}

export function addTelegramURLPrefix(telegram) {
  return `https://t.me/${telegram}`
}

export async function formatEncryptedUserContact(privateKey, profileContact) {
  if (profileContact !== undefined) {
    const decryptedContact = await decryptData(privateKey, profileContact)
    return decryptedContact !== '' ? addTelegramHandlePrefix(decryptedContact) : decryptedContact
  } else {
    return ''
  }
}

export function formatTradesCountInfo(tradesCount) {
  const tradesLabel = tradesCount === 1 ? 'trade' : 'trades'
  return `${tradesCount} ${tradesLabel}`
}

/** UI Elements **/
export function scrollToElement(el) {
  if (el) {
    el.scrollIntoView({ behavior: 'smooth', block: 'center' })
  }
}
