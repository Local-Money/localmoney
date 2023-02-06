import type { Config } from 'mixpanel-browser'
import mixpanel from 'mixpanel-browser'

export function initAnalytics(token: string, config?: Partial<Config>) {
  mixpanel.init(token, config)
}

export function trackPage(page: Page) {
  mixpanel.track(page)
}

export function trackWalletConnection(state: WalletEvents, address: string | undefined = undefined) {
  mixpanel.identify(address)
  mixpanel.track(state)
}

export function trackOffer(event: OfferEvents, offer: OfferData) {
  mixpanel.track(event, offer)
}

export function trackTrade(event: TradeEvents, offer: TradeData) {
  mixpanel.track(event, offer)
}

export function trackSocialLinks(event: ClickLinkEvents) {
  mixpanel.track(event)
}

export enum ClickLinkEvents {
  discord = 'link_discord',
  twitter = 'link_twitter',
  github = 'link_github',
  litepaper = 'link_litepaper',
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
  create = 'open_trade',
}

export interface TradeData {
  trade_id: string
  offer_id: string
  trade_amount: string
  trade_denom: string
  trade_type: string
  trade_usd_value: string
}

export enum OfferEvents {
  create = 'create_offer',
  pause = 'pause_offer',
  archive = 'archive_offer',
  unarchive = 'unarchive_offer',
}

export interface OfferData {
  offer_id: string
  offer_min: string
  offer_max: string
  offer_state: string
  offer_type: string
  offer_fiat: string
  offer_denom: string
  offer_rate: string
}

export enum WalletEvents {
  connected = 'wallet connected',
  disconnected = 'wallet disconnected',
}
