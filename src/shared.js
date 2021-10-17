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

export function scrollToElement(el) {
  if (el) {
    el.scrollIntoView({ behavior: 'smooth' })
  }
}
