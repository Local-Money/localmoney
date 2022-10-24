export class LoadingState {
  isLoading: Boolean
  label: string
  transaction: string
  private constructor(isLoading: Boolean, label: string, transaction: string) {
    this.isLoading = isLoading
    this.label = label
    this.transaction = transaction
  }

  static show(label: string, transaction = ''): LoadingState {
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
  [OfferType.buy]: string
  [OfferType.sell]: string
}

export interface OfferResponse {
  offer: GetOffer
  profile: Profile
}

export interface Profile {
  addr: string
  created_at: Date
  requested_trades_count: number
  released_trades_count: number
  last_trade: Date
  contact?: string
  encryption_key?: string
}

export interface GetOffer {
  id: string
  state: OfferState
  rate: string
  min_amount: string
  max_amount: string
  owner: string
  offer_type: OfferType
  denom: Denom
  fiat_currency: FiatCurrency
  timestamp: number
}

export interface PatchOffer {
  id: string
  state: OfferState
  rate: string
  min_amount: string
  max_amount: string
  owner_contact?: string
  owner_encryption_key?: string
}

export interface PostOffer {
  owner_contact: string
  owner_encryption_key: string
  rate: string
  offer_type: OfferType
  denom: Denom
  fiat_currency: FiatCurrency
  min_amount: string
  max_amount: string
}

export enum FiatCurrency {
  BRL = 'BRL',
  ARS = 'ARS',
  COP = 'COP',
}

export enum OfferState {
  active = 'active',
  paused = 'paused',
  archived = 'archive',
}

export interface FetchOffersArgs {
  fiatCurrency: FiatCurrency
  offerType: OfferType
  denom: Denom
  order: OfferOrder
}
export enum OfferOrder {
  trades_count = 'trades_count',
  price_rate = 'price_rate',
}

export interface NewTrade {
  offer_id: string
  amount: string
  taker: string
  profile_taker_contact: string
  profile_taker_encryption_key: string
  taker_contact: string
}

export interface Trade {
  id: string
  addr: string
  factory_addr: string
  buyer: string
  buyer_contact?: string
  buyer_encryption_key?: string
  seller: string
  seller_contact?: string
  seller_encryption_key: string
  arbitrator?: string | null
  arbitrator_encryption_key: string
  arbitrator_buyer_contact?: string
  arbitrator_seller_contact?: string
  offer_contract: string
  offer_id: string
  created_at: number
  amount: string
  denom: Denom
  state: TradeState
  state_history: TradeStateItem[]
  fiat: FiatCurrency
}

export interface TradeStateItem {
  actor: string
  state: TradeState
  timestamp: number
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
  offer: OfferResponse
  expired: boolean
}

export interface HubConfig {
  local_denom: Denom
  offer_addr: string
  trade_addr: string
  profile_addr: string
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

export interface Arbitrator {
  arbitrator: string
  fiat: FiatCurrency
  encryption_key: string
}
