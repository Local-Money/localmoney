import mixpanel from 'mixpanel-browser'

const ANALYTICS_TOKEN = process.env.ANALYTICS_TOKEN as string

export function initAnalytics() {
  mixpanel.init(ANALYTICS_TOKEN, { debug: true })
}

export function trackPage(page: Page) {
  mixpanel.track(page)
}

export enum Page {
  home = 'home',
  my_offers = 'my offers',
  my_trades = 'my trades',
  disputes = 'disputes',
  trade_detail = 'trade detail',
}
