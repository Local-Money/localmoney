import { defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'
import type { TradeInfo } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { getTradeCounterParty } from '~/shared'

export const useNotificationStore = defineStore({
  id: 'notification',
  state: () => {
    return {
      store: useLocalStorage('notification', new Map<string, Notification[]>()),
      client: useClientStore(),
      badgeCount: 0,
    }
  },
  actions: {
    async fetchNotifications() {
      const trades = await this.client.client.fetchTrades()
      const wallet = this.client.userWallet.address
      const openTrades = trades.filter((tradeInfo) => {
        const counterParty = getTradeCounterParty(wallet, tradeInfo.trade)
        return (
          !tradeInfo.expired &&
          tradeInfo.trade.state === TradeState.request_created &&
          tradeInfo.offer.owner !== counterParty
        )
      })
      const notifications = this.store.get(wallet) ?? []
      openTrades.forEach((tradeInfo) => {
        const found =
          notifications.find((n) => n.id === tradeInfo.trade.id && n.state === tradeInfo.trade.state) !== undefined
        if (!found) {
          notifications.push(mapTradeInfoToNotification(wallet, tradeInfo))
        }
      })
      await this.addNotifications(notifications)
    },
    async addNotifications(notifications: Notification[]) {
      const wallet = this.client.userWallet.address
      this.badgeCount = notifications.filter((notification) => !notification.isAlreadyRead).length
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
      }
    },
    async markAllAsRead() {
      const notifications = this.notifications()
      notifications.forEach((notification) => (notification.isAlreadyRead = true))
      this.badgeCount = 0
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

function mapTradeInfoToNotification(wallet: string, tradeInfo: TradeInfo): Notification {
  const state = tradeInfo.trade.state
  const id = tradeInfo.trade.id
  const sender = getTradeCounterParty(wallet, tradeInfo.trade)
  const message = getMessageByState(state)
  const time = Date.now()
  const isAlreadyRead = false
  return { state, id, message, sender, time, isAlreadyRead }
}

// TODO define message for each state
function getMessageByState(_: TradeState): string {
  return 'You have a open trade'
}

export interface Notification {
  id: string
  message: string
  sender: string
  state: TradeState
  time: number
  isAlreadyRead: boolean
}
