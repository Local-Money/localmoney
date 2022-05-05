/** String Formatting **/
export function formatAddress(address) {
    const start = address.substr(0, 8);
    const length = address.length;
    const end = address.substr(length - 6, length - 1);
    return `${start}...${end}`;
}

export function formatAmount(amount, ustAmount = true) {
    if (ustAmount) {
        amount = amount / 1000000;
    }
    return amount.toFixed(2);
}

export function formatFiatAmount(amount) {
    return amount.toFixed(2);
}

export function formatTradeState(state) {
    return {
        request_created: "Expired",
        request_accepted: "Expired",
        escrow_funded: "Expired",
        request_expired: "Expired",
        request_canceled: "Canceled",
        escrow_refunded: "Refunded",
        escrow_released: "Completed",
        settled_for_maker: "settled_for_maker",
        settled_for_taker: "settled_for_taker",
    }[state];
}

export function formatDate(date, showTime = true) {
    let year = new Intl.DateTimeFormat("en", { year: "numeric" }).format(date);
    let month = new Intl.DateTimeFormat("en", { month: "short" }).format(date);
    let day = new Intl.DateTimeFormat("en", { day: "2-digit" }).format(date);
    if (showTime) {
        let time = new Intl.DateTimeFormat("en", {
            hour: "2-digit",
            minute: "2-digit",
        }).format(date);
        return `${day} ${month} ${year} - ${time}`;
    } else {
        return `${day} ${month} ${year}`;
    }
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
    return trade.state === "created" && trade.seller === walletAddr;
}

export function tradeCanBeReleased(tradeInfo, walletAddr) {
    const { trade } = tradeInfo;
    return trade.state === "escrow_funded" && trade.seller === walletAddr;
}

export function tradeCanBeRefunded(tradeInfo, walletAddr) {
    const { trade } = tradeInfo;
    return (
        trade.state === "escrow_funded" &&
        tradeInfo.expired &&
        trade.sender === walletAddr
    );
}

export function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
