export class LoadingState {
  isLoading: Boolean
  label: String
  transaction: String
  private constructor(isLoading: Boolean, label: String, transaction: String) {
    this.isLoading = isLoading
    this.label = label
    this.transaction = transaction
  }

  static show(label: String, transaction: String = ''): LoadingState {
    return new LoadingState(true, label, transaction)
  }

  static dismiss(): LoadingState {
    return new LoadingState(false, '', '')
  }
}

export enum OfferType {
  buy = 'buy',
  sell = 'sell',
}

export interface OfferTypeLabel {
  [OfferType.buy]: String
  [OfferType.sell]: String
}

export interface GetOffer extends PatchOffer {
  id: String
  owner: String
  rate: String
  denom: Denom
  min_amount: string
  max_amount: string
  offer_type: OfferType
  fiat_currency: FiatCurrency
  state: OfferState
  timestamp: number
  last_traded_at: number
  trades_count: number
}

export interface PatchOffer {
  id: String
  state: OfferState
  rate: String
  min_amount: string
  max_amount: string
}

export interface PostOffer {
  offer_type: OfferType
  fiat_currency: FiatCurrency
  rate: String
  denom: Denom
  min_amount: string
  max_amount: string
  maker_contact: String
}

export enum FiatCurrency { BRL = 'BRL', ARS = 'ARS', COP = 'COP' }

export enum OfferState { active = 'active', paused = 'paused', archived = 'archive' }

export interface FetchOffersArgs {
  fiatCurrency: FiatCurrency
  offerType: OfferType
  denom: Denom
}

export interface NewTrade {
  offer_id: string
  amount: string
  taker: string // TODO
  // counterparty: string,
  // taker_contact: string, //TODO
  // arbitrator: TODO,
}

export interface Trade {
  id: string
  addr: string
  factory_addr: string
  buyer: string
  seller: string
  taker_contact: string
  arbitrator?: string | null
  offer_contract: string
  offer_id: string
  created_at: number
  amount: string
  denom: Denom
  state: TradeState
  asset: FiatCurrency
}

export enum TradeState {
  request_created = 'request_created',
  request_accepted = 'request_accepted',
  request_canceled = 'request_canceled',
  request_expired = 'request_expired',
  escrow_funded = 'escrow_funded',
  escrow_refunded = 'escrow_refunded',
  fiat_deposited = 'fiat_deposited',
  escrow_released = 'escrow_released',
  escrow_disputed = 'escrow_disputed',
  settled_for_maker = 'settled_for_maker',
  settled_for_taker = 'settled_for_taker',
}

export interface TradeInfo {
  trade: Trade
  offer: GetOffer
  expired: boolean
}

export interface HubConfig {
  local_denom: Denom
  offer_addr: string
  trade_addr: string
  trading_incentives_addr: string
  local_market_addr: string
}

export interface Denom {
  native: string
}

export interface UserWallet {
  isConnected: boolean
  address: string
}
