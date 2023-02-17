import { defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'
import { TradeState } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'

export const useNotificationStore = defineStore({
  id: 'notification',
  state: () => {
    return {
      store: useLocalStorage('notification', new Map<string, Notification[]>()),
      lastSeen: useLocalStorage('last_seem', new Map<string, number>()),
      client: useClientStore(),
      badgeCount: 0,
    }
  },
  actions: {
    async fetchNotifications(limit = 100, last?: number) {
      await this.cleanNotification()
      let trades = await this.client.client.fetchTrades(limit, last)
      const disputes = await this.client.client.fetchDisputedTrades(limit, last)
      const openDisputes = disputes.openDisputes.length > 0 ? disputes.openDisputes : []
      trades = trades.concat(openDisputes)
      const wallet = this.client.userWallet.address
      const lastSeen = this.lastSeen.get(wallet) ?? 0
      let notifications = this.notifications()
      trades.forEach((tradeInfo) => {
        if (!tradeInfo.expired) {
          if (tradeInfo.trade.arbitrator === wallet) {
            const dispute = tradeInfo.trade.state_history.find((state) => state.state === TradeState.escrow_disputed)
            tradeInfo.trade.state_history = dispute ? [dispute] : []
          }
          tradeInfo.trade.state_history.forEach((state) => {
            const time = state.timestamp * 1000
            if (time > lastSeen && state.actor !== wallet) {
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
        this.lastSeen.set(wallet, Date.now())
      }
    },
    async markAllAsRead() {
      const wallet = this.client.userWallet.address
      const notifications = this.store.get(wallet) ?? []
      notifications.forEach((n) => {
        n.isAlreadyRead = true
      })
      this.addNotifications(notifications)
      this.lastSeen.set(wallet, Date.now())
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

function toNotification(id: number, state: TradeState, sender: string, time: number): Notification {
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
    case TradeState.escrow_canceled:
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
      return 'Dispute has been requested'
    case TradeState.settled_for_maker:
      return 'Dispute resolved'
    case TradeState.settled_for_taker:
      return 'Dispute resolved'
  }
}

export interface Notification {
  id: number
  message: string
  sender: string
  state: TradeState
  time: number
  isAlreadyRead: boolean
}
