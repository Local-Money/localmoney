import type { Notification } from '~/stores/notification'
import { useNotificationStore } from '~/stores/notification'

const interval = 10 * 1000

class NotificationHandler {
  private store
  private interval?: NodeJS.Timer

  constructor() {
    this.store = useNotificationStore()
  }

  public async register() {
    await this.handle()
    this.interval = setInterval(() => {
      this.handle()
    }, interval)
  }

  public async unregister() {
    await this.store.cleanNotification()
    clearInterval(this.interval)
  }

  public readNotification(notification: Notification) {
    this.store.markAsRead(notification)
  }

  public async readAllNotification() {
    await this.store.markAllAsRead()
  }

  public notifications(): Notification[] {
    return this.store.notifications().reverse()
  }

  public notificationCount(): number {
    return this.store.badgeCount
  }

  private async handle() {
    await this.store.fetchNotifications()
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
