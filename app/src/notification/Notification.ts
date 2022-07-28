import type { Notification } from '~/stores/notification'
import { useNotificationStore } from '~/stores/notification'
import { useClientStore } from '~/stores/client'
import type { TradeInfo } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'

const millisecond = 1000

class NotificationHandler {
  readonly badgeCount
  private store
  private client
  private interval?: NodeJS.Timer

  constructor() {
    this.store = useNotificationStore()
    this.client = useClientStore()
    console.log(this.client)
    this.badgeCount = computed(() => this.store.badgeCount)
  }

  public async register() {
    this.interval = setInterval(() => {
      this.handle()
    }, millisecond)
  }

  public async unregister() {
    clearInterval(this.interval)
  }

  private async handle() {
    await this.client.fetchTrades()
    if (this.client.trades.isSuccess()) {
      const notifications = this.client.trades.data
        .filter(tradeInfo => this.filterOpenTrades(tradeInfo))
        .map(tradeInfo => this.mapTradeInfoToNotification(tradeInfo))
      await this.store.addNotifications(this.client.userWallet.address, notifications)
    }
  }

  private filterOpenTrades(tradeInfo: TradeInfo): boolean {
    // const notifications = this.store.notifications(this.client.userWallet.address)
    // const isOnNotifications = notifications.find(notification => notification.id === tradeInfo.trade.id) !== undefined
    // console.log(isOnNotifications)
    return this.verifyOpenTrade(tradeInfo)
  }

  private verifyOpenTrade(tradeInfo: TradeInfo): boolean {
    return !tradeInfo.expired && [
      TradeState.request_created,
      TradeState.request_accepted,
      TradeState.escrow_funded,
      TradeState.fiat_deposited,
      TradeState.escrow_disputed,
    ].includes(tradeInfo.trade.state)
  }

  private mapTradeInfoToNotification(tradeInfo: TradeInfo): Notification {
    return {
      state: tradeInfo.trade.state,
      id: tradeInfo.trade.id,
      message: '',
      sender: '',
    }
  }
}

let notificationHandler: NotificationHandler

const useNotificationSystem = () => {
  if (!notificationHandler) {
    notificationHandler = new NotificationHandler()
  }
  return notificationHandler
}

export default useNotificationSystem
