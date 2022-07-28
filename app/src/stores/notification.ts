import { defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'
import type { TradeState } from '~/types/components.interface'

export const useNotificationStore = defineStore({
  id: 'notification',
  state: () => {
    return {
      store: useLocalStorage('notification', new Map<string, Notification[]>()),
      badgeCount: 0,
    }
  },
  actions: {
    async addNotification(owner: string, notification: Notification) {
      const notifications = this.notifications(owner)
      notifications.push(notification)
      await this.addNotifications(owner, notifications)
    },
    async addNotifications(owner: string, notifications: Notification[]) {
      this.badgeCount = notifications.length
      this.store.set(owner, notifications)
    },
    notifications(owner: string): Notification[] {
      return this.store.get(owner) ?? []
    },
    async cleanNotification(owner: string) {
      this.badgeCount = 0
      this.store.delete(owner)
    },
  },
})

export interface Notification {
  id: string
  message: string
  sender: string
  state: TradeState
}
