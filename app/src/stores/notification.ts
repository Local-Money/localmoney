import { defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'
import { TradeState } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'

export const useNotificationStore = defineStore({
  id: 'notification',
  state: () => {
    return {
      store: useLocalStorage('notification', new Map<string, Notification[]>()),
      lastSeem: useLocalStorage('last_seem', new Map<string, number>()),
      client: useClientStore(),
      badgeCount: 0,
    }
  },
  actions: {
    async fetchNotifications() {
      const trades = await this.client.client.fetchTrades()
      const wallet = this.client.userWallet.address
      const newNotifications: Notification[] = []
      const lastSeem = this.lastSeem.get(wallet) ?? 0
      const notifications = this.notifications()
      trades.forEach((tradeInfo) => {
        if (!tradeInfo.expired) {
          tradeInfo.trade.state_history.reverse().forEach((state) => {
            const time = state.timestamp * 1000
            if (time > lastSeem && state.actor !== wallet) {
              const notification = toNotification(tradeInfo.trade.id, state.state, state.actor, time)
              if (!notifications.includes(notification)) {
                newNotifications.push(notification)
              }
            }
          })
        }
      })
      await this.addNotifications(newNotifications)
    },
    async addNotifications(notifications: Notification[]) {
      this.cleanNotification()
      const wallet = this.client.userWallet.address
      this.badgeCount = notifications.length
      this.store.set(wallet, notifications)
    },
    notifications(): Notification[] {
      const wallet = this.client.userWallet.address
      return this.store.get(wallet)?.filter((notification) => !notification.isAlreadyRead) ?? []
    },
    async markAsRead(notification: Notification) {
      const wallet = this.client.userWallet.address
      const notifications = this.store.get(wallet) ?? []
      const index = notifications.indexOf(notification)
      if (index > -1) {
        notifications[index].isAlreadyRead = true
        await this.addNotifications(notifications)
        this.badgeCount--
        this.lastSeem.set(wallet, Date.now())
      }
    },
    cleanNotification() {
      const notifications = this.notifications()
      notifications.forEach((notification, index) => {
        if (notification.isAlreadyRead) {
          notifications.slice(index, 1)
        }
      })
    },
  },
})

function toNotification(id: string, state: TradeState, sender: string, time: number): Notification {
  const message = getMessageByState(state)
  return { state, id, message, sender, time, isAlreadyRead: false }
}

// TODO define message for each state
function getMessageByState(state: TradeState): string {
  switch (state) {
    case TradeState.request_created:
      return 'You have a new Trade request'
    case TradeState.request_accepted:
      return 'Your Trade was accepted'
    case TradeState.request_canceled:
      return 'Your Trade was canceled'
    case TradeState.request_expired:
      return 'A Trade has been expired'
    case TradeState.escrow_funded:
      return 'Your Trade has funds in the escrow'
    case TradeState.escrow_refunded:
      return 'escrow_refunded'
    case TradeState.fiat_deposited:
      return 'The Trade was marked as fiat deposited'
    case TradeState.escrow_released:
      return 'Trade finished with success'
    case TradeState.escrow_disputed:
      return 'The trade is in dispute'
    case TradeState.settled_for_maker:
      return 'The dispute was decided in favor of the maker'
    case TradeState.settled_for_taker:
      return 'The dispute was decided in favor of the taker'
  }
}

export interface Notification {
  id: string
  message: string
  sender: string
  state: TradeState
  time: number
  isAlreadyRead: boolean
}
