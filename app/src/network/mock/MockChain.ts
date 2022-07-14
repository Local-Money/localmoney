import offers from '~/network/mock/fixtures/home/offers.json'
import myOffers from '~/network/mock/fixtures/offers/my-offers.json'
import myTrades from '~/network/mock/fixtures/trades/my-trades.json'
import trade from '~/network/mock/fixtures/trade/open-trade-info.json'
import type {
  Denom,
  FetchOffersArgs,
  GetOffer,
  PatchOffer,
  PostOffer,
  Trade,
  TradeInfo
} from '~/types/components.interface'
import type { Chain } from '~/network/Chain'
import {NewTrade, TradeState} from "~/types/components.interface";

class MockChain implements Chain {
  private ownerPublicKey = 'terra13zelgwgwuj0lqw3xdgn32l996tfzjfayetqjs7'
  
  async init() {
    return await someDelay()
  }
  
  async connectWallet() {
    await someDelay()
  }
  
  getWalletAddress(): string {
    return this.ownerPublicKey
  }

  async fetchOffers({ fiatCurrency, offerType }: FetchOffersArgs) {
    await someDelay()
    console.log('offers :>> ', offers)
    console.log(offerType)
    console.log('fiatCurrency :>> ', fiatCurrency)
    return offers.filter(offer => offer.offer_type === offerType).filter(offer => offer.fiat_currency === fiatCurrency) as GetOffer[]
  }

  async fetchMyOffers() {
    await someDelay()
    return myOffers.filter(offer => offer.owner === this.ownerPublicKey) as GetOffer[]
  }

  async createOffer(postOffer: PostOffer) {
    console.log('postOffer :>> ', postOffer)
    await someDelay()
  }

  async updateOffer(updateOffer: PatchOffer) {
    console.log('postOffer :>> ', updateOffer)
    await someDelay()
  }

  async openTrade(trade: NewTrade) {
    await someDelay()
    return '110_2_2'
  }

  async fetchTrades() {
    await someDelay()
    return myTrades as TradeInfo[]
  }

  async fetchTradeDetail(tradeId: string) {
    console.log(tradeId)
    await someDelay()
    return trade as Trade
  }

  async acceptTradeRequest(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.request_accepted)
  }

  async cancelTradeRequest(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.request_canceled)
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async fundEscrow(tradeId: string, amount: string, denom: Denom) {
    await this.changeTradeState(tradeId, TradeState.escrow_funded)
  }

  async setFiatDeposited(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.fiat_deposited)
  }

  async openDispute(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.escrow_disputed)
  }

  async refundEscrow(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.escrow_refunded)
  }

  async releaseEscrow(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.escrow_released)
  }

  private async changeTradeState(tradeId: string, state: TradeState) {
    const selectedTrade = myTrades.find(trade => trade.trade.addr === tradeId)
    if (selectedTrade)
      selectedTrade.trade.state = state

    trade.state = state
  }
}

function sleep(ms: number) {
  console.log('sleep', ms)
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function someDelay(min = 800, max = 1800) {
  return await sleep(Math.floor(Math.random() * (max - min) + min))
}

export default MockChain
