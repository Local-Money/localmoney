import type { Notification } from '~/stores/notification'
import { useNotificationStore } from '~/stores/notification'

const millisecond = 15000

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
    }, millisecond)
  }

  public async unregister() {
    await this.store.cleanNotification()
    clearInterval(this.interval)
  }

  public async readNotification(notification: Notification) {
    await this.store.markAsRead(notification)
  }

  public async readAllNotifications() {
    await this.store.markAllAsRead()
  }

  public notifications(): Notification[] {
    return this.store.notifications()
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
  if (!notificationHandler)
    notificationHandler = new NotificationHandler()
  return notificationHandler
}

export default useNotificationSystem
