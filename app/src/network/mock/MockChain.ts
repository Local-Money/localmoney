import offers from '~/network/mock/fixtures/home/offers.json'
import myOffers from '~/network/mock/fixtures/offers/my-offers.json'
import myTrades from '~/network/mock/fixtures/trades/my-trades.json'
import trade from '~/network/mock/fixtures/trade/open-trade-info.json'
import type {
  Arbitrator,
  Denom,
  FetchOffersArgs,
  GetOffer,
  NewTrade,
  PatchOffer,
  PostOffer,
  Trade,
  TradeInfo,
} from '~/types/components.interface'
import type { Chain } from '~/network/Chain'
import { TradeState } from '~/types/components.interface'

export class MockChain implements Chain {
  async init() {
    return await someDelay()
  }

  async connectWallet() {
    await someDelay()
  }

  getWalletAddress(): string {
    return 'terra13zelgwgwuj0lqw3xdgn32l996tfzjfayetqjs7'
  }

  async fetchOffers({ fiatCurrency, offerType }: FetchOffersArgs) {
    await someDelay()
    console.log('offers :>> ', offers)
    console.log(offerType)
    console.log('fiatCurrency :>> ', fiatCurrency)
    return offers
      .filter((offer) => offer.offer_type === offerType)
      .filter((offer) => offer.fiat_currency === fiatCurrency) as GetOffer[]
  }

  async fetchMyOffers() {
    await someDelay()
    return myOffers.filter((offer) => offer.owner === this.getWalletAddress()) as GetOffer[]
  }

  async createOffer(postOffer: PostOffer) {
    console.log('postOffer :>> ', postOffer)
    await someDelay()
  }

  async updateOffer(updateOffer: PatchOffer) {
    console.log('postOffer :>> ', updateOffer)
    await someDelay()
  }

  async openTrade(_: NewTrade) {
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

  async acceptTradeRequest(tradeId: string, makerContact?: string) {
    console.log(makerContact)
    await this.changeTradeState(tradeId, TradeState.request_accepted)
  }

  async cancelTradeRequest(tradeId: string) {
    await this.changeTradeState(tradeId, TradeState.request_canceled)
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async fundEscrow(tradeId: string, amount: string, denom: Denom, makerContact?: string) {
    console.log(makerContact)
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
    const selectedTrade = myTrades.find((trade) => trade.trade.addr === tradeId)
    if (selectedTrade) {
      selectedTrade.trade.state = state
    }
    trade.state = state
  }

  newArbitrator(_arbitrator: Arbitrator): Promise<void> {
    return Promise.resolve(undefined)
  }

  settleDispute(_tradeId: string, _winner: string): Promise<void> {
    return Promise.resolve(undefined)
  }

  fetchOffer(offerId: string): Promise<GetOffer> {
    const offer = offers[0] as GetOffer
    offer.id = offerId
    return Promise.resolve(offer)
  }

  fetchDisputedTrades(): Promise<{ openDisputes: TradeInfo[]; closedDisputes: TradeInfo[] }> {
    const openDisputes = [myTrades[0] as TradeInfo]
    const closedDisputes = [] as TradeInfo[]
    return Promise.resolve({ openDisputes, closedDisputes })
  }

  fetchArbitrators(): Promise<Arbitrator[]> {
    const offer = offers[0] as GetOffer
    return Promise.resolve([
      {
        arbitrator: 'kujira1qeruy425sxl9lcu6ahjh58u89qh03y7xcplmn6',
        fiat: offer.fiat_currency,
      },
    ])
  }
}

function sleep(ms: number) {
  console.log('sleep', ms)
  return new Promise((resolve) => setTimeout(resolve, ms))
}

async function someDelay(min = 800, max = 1800) {
  return await sleep(Math.floor(Math.random() * (max - min) + min))
}
