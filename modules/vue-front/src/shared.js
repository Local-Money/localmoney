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

export function formatDate(date) {
  let year = new Intl.DateTimeFormat("en", { year: "numeric" }).format(
    date
  );
  let month = new Intl.DateTimeFormat("en", { month: "short" }).format(
    date
  );
  let day = new Intl.DateTimeFormat("en", { day: "2-digit" }).format(date);
  return `${day} ${month} ${year}`;
}

/** UI Elements **/
export function scrollToElement(el) {
  if (el) {
    el.scrollIntoView({ behavior: "smooth", block: "center" });
  }
}

/** Trade State **/
export function tradeCanBeFunded(tradeInfo, walletAddr) {
  const { trade } = tradeInfo;
  return trade.state === 'created' && trade.sender === walletAddr;
}

export function tradeCanBeReleased(tradeInfo, walletAddr) {
  const { trade } = tradeInfo;
  return trade.state === 'escrow_funded' && trade.sender === walletAddr
}
export function tradeCanBeRefunded(tradeInfo, walletAddr) {
  const { trade } = tradeInfo;
  return trade.state === 'escrow_funded'
    && tradeInfo.expired
    && trade.sender === walletAddr;
}