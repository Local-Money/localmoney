import type { Notification } from '~/stores/notification'
import { useNotificationStore } from '~/stores/notification'
import { useClientStore } from '~/stores/client'
import type { TradeInfo } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'

const millisecond = 15000

class NotificationHandler {
  readonly badgeCount
  private store
  private client
  private interval?: NodeJS.Timer

  constructor() {
    this.store = useNotificationStore()
    this.client = useClientStore()
    this.badgeCount = computed(() => this.store.badgeCount)
  }

  public async register() {
    await this.handle()
    this.interval = setInterval(() => {
      this.handle()
    }, millisecond)
  }

  public async unregister() {
    clearInterval(this.interval)
  }

  public async readAllNotifications() {
    const userWallet = this.client.userWallet.address
    await this.store.cleanNotification(userWallet)
  }

  private async handle() {
    await this.client.fetchTrades()
    if (this.client.trades.isSuccess()) {
      const userWallet = this.client.userWallet.address
      const trades = this.client.trades.data
      const openTrades = trades.filter(this.filterOpenTrades)
      const notifications = this.store.notifications(userWallet)
      const newNotifications: Notification[] = openTrades.filter((tradeInfo) => {
        return notifications.find((notification) => {
          return notification.id !== tradeInfo.trade.id
              || (notification.id === tradeInfo.trade.id && notification.state !== tradeInfo.trade.state)
        }) !== undefined || notifications.length <= 0
      }).map(tradeInfo => this.mapTradeInfoToNotification(tradeInfo))
      const notificationsUpdated = [...notifications, ...newNotifications]
      console.log(notificationsUpdated)
      await this.store.addNotifications(this.client.userWallet.address, notificationsUpdated)
    }
  }

  private filterOpenTrades(tradeInfo: TradeInfo): boolean {
    return !tradeInfo.expired && [
      TradeState.request_created,
      TradeState.request_accepted,
      TradeState.escrow_funded,
      TradeState.fiat_deposited,
      TradeState.escrow_disputed,
    ].includes(tradeInfo.trade.state)
  }

  private mapTradeInfoToNotification(tradeInfo: TradeInfo): Notification {
    const state = tradeInfo.trade.state
    const id = tradeInfo.trade.id
    const sender = this.getCounterParty(tradeInfo)
    const message = `You have a trade (id: ${id}) with ${sender} on state ${state}`
    return { state, id, message, sender }
  }

  private getCounterParty(tradeInfo: TradeInfo): string{
    const trade = tradeInfo.trade
    const walletAddress = this.client.userWallet.address
    return walletAddress === trade.seller ? trade.buyer : trade.seller
  }
}

let notificationHandler: NotificationHandler

const useNotificationSystem = () => {
  if (!notificationHandler)
    notificationHandler = new NotificationHandler()
  return notificationHandler
}

export default useNotificationSystem
