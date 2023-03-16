import type { Config } from 'mixpanel-browser'
import mixpanel from 'mixpanel-browser'
import type { GetOffer, PatchOffer, PostOffer, Trade } from '~/types/components.interface'
import { denomToValue, microDenomToDisplay } from '~/utils/denom'
import { CRYPTO_DECIMAL_PLACES } from '~/utils/constants'
import type { ChainClient } from '~/network/Chain'

const TRADE = 'trade'

export function initAnalytics(token: string, config?: Partial<Config>) {
  mixpanel.init(token, config)
}

export function trackPage(page: Page, data?: AppData) {
  mixpanel.track(page, data)
}

export function trackWalletConnection(events: WalletEvents, address?: string) {
  if (address) {
    mixpanel.identify(address)
  }
  mixpanel.track(events)
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

export function trackAppEvents(event: AppEvents, data?: AppData) {
  mixpanel.track(event, data)
}

export enum Page {
  home = 'page_home',
  my_offers = 'page_my_offers',
  my_trades = 'page_my_trades',
  disputes = 'page_disputes',
  trade_detail = 'page_trade_detail',
  maker = 'page_maker',
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
  trade_amount: number
  trade_denom: string
  trade_type: string
  trade_state: string
  trade_maker: string
  trade_taker: string
}

export function toTradeData(trade: Trade, offer: GetOffer, chainClient: ChainClient): TradeData {
  let trade_maker: string
  let trade_taker: string

  if (trade.buyer === offer.owner) {
    trade_maker = trade.buyer
    trade_taker = trade.seller
  } else {
    trade_maker = trade.seller
    trade_taker = trade.buyer
  }

  return {
    trade_id: trade.id,
    offer_id: trade.offer_id,
    trade_amount: Number(trade.amount) / CRYPTO_DECIMAL_PLACES,
    trade_denom: microDenomToDisplay(denomToValue(trade.denom), chainClient),
    trade_type: offer.offer_type,
    trade_state: trade.state,
    trade_maker,
    trade_taker,
  }
}

export enum OfferEvents {
  created = 'offer_created',
  updated = 'offer_updated',
  unarchived = 'offer_unarchived',
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

export function toOfferData(offerId: number, offer: PostOffer | PatchOffer, chainClient: ChainClient): OfferData {
  const offer_denom = 'denom' in offer ? microDenomToDisplay(denomToValue(offer.denom), chainClient) : undefined
  return {
    offer_id: offerId,
    offer_max: Number(offer.max_amount) / CRYPTO_DECIMAL_PLACES,
    offer_min: Number(offer.min_amount) / CRYPTO_DECIMAL_PLACES,
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

export enum ClickLinkEvents {
  discord = 'link_discord',
  twitter = 'link_twitter',
  github = 'link_github',
  litepaper = 'link_litepaper',
  medium = 'link_medium',
}

export enum AppEvents {
  list_offers = 'list_offers_filter',
  open_notifications = 'click_open_notifications',
  close_notifications = 'click_close_notifications',
  click_notification = 'click_notification',
  clear_all_notifications = 'click_clear_all_notifications',
}

export type AppData = Record<string, any>
