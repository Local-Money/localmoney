import type { Config } from 'mixpanel-browser'
import mixpanel from 'mixpanel-browser'

export function initAnalytics(token: string, config?: Partial<Config>) {
  mixpanel.init(token, config)
}

export function trackPage(page: Page) {
  mixpanel.track(page)
}

export function trackWalletConnection(state: WalletState, address: string | undefined = undefined) {
  mixpanel.identify(address)
  mixpanel.track(state)
}

export enum Page {
  home = 'home',
  my_offers = 'my offers',
  my_trades = 'my trades',
  disputes = 'disputes',
  trade_detail = 'trade detail',
}

export enum Events {}

export enum WalletState {
  connected = 'wallet connected',
  disconnected = 'wallet disconnected',
}
