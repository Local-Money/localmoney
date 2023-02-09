import type { Config } from 'mixpanel-browser'
import mixpanel from 'mixpanel-browser'
import type { OfferType, PatchOffer, PostOffer, Trade } from '~/types/components.interface'
import { denomToValue, microDenomToDenom } from '~/utils/denom'

const TRADE = 'trade'

export function initAnalytics(token: string, config?: Partial<Config>) {
  mixpanel.init(token, config)
}

export function trackPage(page: Page) {
  mixpanel.track(page)
}

export function trackWalletConnection(state: WalletEvents, address: string | undefined = undefined) {
  if (address) {
    mixpanel.identify(address)
  }
  mixpanel.track(state)
}

export function trackOffer(event: OfferEvents, offer: OfferData) {
  mixpanel.track(event, offer)
}

export function trackTrade(event: TradeEvents, trade: TradeData) {
  mixpanel.track(event, trade)
  mixpanel.track(TRADE, trade)
}

export function trackSocialLinks(event: ClickLinkEvents) {
  mixpanel.track(event)
}

export enum ClickLinkEvents {
  discord = 'link_discord',
  twitter = 'link_twitter',
  github = 'link_github',
  gitbook = 'link_gitbook',
  medium = 'link_medium',
}
export enum Page {
  home = 'page_home',
  my_offers = 'page_my_offers',
  my_trades = 'page_my_trades',
  disputes = 'page_disputes',
  trade_detail = 'page_trade_detail',
}

export enum TradeEvents {
  created = 'trade_created',
  accepted = 'trade_accepted',
  canceled = 'trade_canceled',
  funded = 'trade_escrow_funded',
  paid = 'trade_fiat_deposited',
  released = 'trade_escrow_released',
  refunded = 'trade_escrow_refunded',
  disputed = 'trade_escrow_disputed',
  dispute_settled = 'trade_dispute_settled',
}

export interface TradeData {
  trade_id: number
  offer_id: number
  trade_amount: string
  trade_denom: string
  trade_type: string
  trade_state: string
}

export function toTradeData(trade: Trade, offerType: OfferType): TradeData {
  return {
    trade_id: trade.id,
    offer_id: trade.offer_id,
    trade_amount: trade.amount,
    trade_denom: microDenomToDenom(denomToValue(trade.denom)),
    trade_type: offerType,
    trade_state: trade.state,
  }
}

export enum OfferEvents {
  created = 'offer_created',
  updated = 'offer_updated',
  unarchived = 'unarchive_offer',
}

export interface OfferData {
  offer_id: number
  offer_min?: number
  offer_max?: number
  offer_state?: string
  offer_type?: string
  offer_fiat?: string
  offer_denom?: string
  offer_rate?: string
}

export function toOfferData(offerId: number, offer: PostOffer | PatchOffer): OfferData {
  const offer_denom = 'denom' in offer ? microDenomToDenom(denomToValue(offer.denom)) : undefined
  return {
    offer_id: offerId,
    offer_max: Number(offer.max_amount),
    offer_min: Number(offer.min_amount),
    offer_rate: offer.rate,
    offer_state: (offer as PatchOffer).state ?? null,
    offer_type: (offer as PostOffer).offer_type ?? null,
    offer_fiat: (offer as PostOffer).fiat_currency ?? null,
    offer_denom,
  }
}

export enum WalletEvents {
  connected = 'wallet_connected',
  disconnected = 'wallet_disconnected',
}
