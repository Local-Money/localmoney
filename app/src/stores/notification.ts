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
      const lastSeem = this.lastSeem.get(wallet) ?? 0
      let notifications = this.notifications()
      trades.forEach((tradeInfo) => {
        if (!tradeInfo.expired) {
          tradeInfo.trade.state_history.forEach((state) => {
            const time = state.timestamp * 1000
            if (time > lastSeem && state.actor !== wallet) {
              const notification = toNotification(tradeInfo.trade.id, state.state, state.actor, time)
              const found = notifications.find(
                (n) =>
                  n.id === notification.id &&
                  n.state === notification.state &&
                  n.time === notification.time &&
                  n.sender === notification.sender
              )
              if (!found) {
                notifications.push(notification)
              }
            }
          })
        } else {
          // clears the storage of notifications of expired trades that have been read
          notifications = notifications.filter((n) => n.id !== tradeInfo.trade.id && n.isAlreadyRead) ?? notifications
        }
      })
      this.addNotifications(notifications)
    },
    addNotifications(notifications: Notification[]) {
      const wallet = this.client.userWallet.address
      this.store.set(wallet, notifications)
      this.badgeCount = this.notifications().length
    },
    notifications(): Notification[] {
      const wallet = this.client.userWallet.address
      return this.store.get(wallet)?.filter((notification) => !notification.isAlreadyRead) ?? []
    },
    markAsRead(notification: Notification) {
      const wallet = this.client.userWallet.address
      const notifications = this.store.get(wallet) ?? []
      const index = notifications.indexOf(notification)
      if (index > -1) {
        notifications[index].isAlreadyRead = true
        this.badgeCount--
        this.addNotifications(notifications)
        this.lastSeem.set(wallet, Date.now())
      }
    },
    async markAllAsRead() {
      const wallet = this.client.userWallet.address
      const notifications = this.store.get(wallet) ?? []
      notifications.forEach((n) => {
        n.isAlreadyRead = true
      })
      this.addNotifications(notifications)
      this.lastSeem.set(wallet, Date.now())
    },
    async cleanNotification() {
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

function getMessageByState(state: TradeState): string {
  switch (state) {
    case TradeState.request_created:
      return 'New trade request'
    case TradeState.request_accepted:
      return 'Trade request was accepted'
    case TradeState.request_canceled:
      return 'Trade has been canceled'
    case TradeState.request_expired:
      return 'Trade expired'
    case TradeState.escrow_funded:
      return 'Trade has been funded'
    case TradeState.escrow_refunded:
      return 'Trade refunded'
    case TradeState.fiat_deposited:
      return 'Fiat deposited'
    case TradeState.escrow_released:
      return 'Trade finished successfully'
    case TradeState.escrow_disputed:
      return 'Trade in dispute'
    case TradeState.settled_for_maker:
      return 'Dispute resolved'
    case TradeState.settled_for_taker:
      return 'Dispute resolved'
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
