<script setup lang="ts">
import { useRouter } from 'vue-router'
import useNotificationSystem from '~/notification/Notification'
import type { Notification } from '~/stores/notification'
import { formatAddress, timeSince } from '~/shared'
import { AppEvents, trackAppEvents } from '~/analytics/analytics'

const emit = defineEmits<{
  (e: 'toggleWidget'): void
}>()

const router = useRouter()
const notification = useNotificationSystem()

async function showTrade(n: Notification) {
  await notification.readNotification(n)
  await router.push({
    name: 'TradeDetail',
    params: { id: n.id },
  })
  emit('toggleWidget')
  trackAppEvents(AppEvents.click_notification, { trade_id: n.id })
}

async function readAll() {
  await notification.readAllNotification()
  trackAppEvents(AppEvents.clear_all_notifications)
}
</script>

<template>
  <div class="wrap-widget">
    <div class="header">
      <p class="title">Notifications</p>
      <p class="mark-read" @click="readAll()">Mark all as read</p>
    </div>
    <div class="content">
      <ul v-if="notification.notificationCount() > 0">
        <li v-for="n in notification.notifications()" :key="`${n.id}_${n.state}`" class="item" @click="showTrade(n)">
          <svg class="icon" viewBox="0 0 24 24" fill="none">
            <path
              d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
              stroke="inherit"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M12.01 8L12.01 12"
              stroke="inherit"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M12.01 16L12 16"
              stroke="inherit"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          <div class="wrap">
            <p class="status">
              {{ n.message }}
            </p>
            <p class="addr">with {{ formatAddress(n.sender) }}</p>
          </div>
          <p class="timestamp">
            {{ timeSince(n.time) }}
          </p>
        </li>
      </ul>
      <div v-else class="empty-state">
        <p>Nothing new here.</p>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.wrap-widget {
  position: fixed;
  width: 100%;
  height: 400px;
  right: 0;
  bottom: 32px;
  z-index: 100;
  background-color: $surface;
  border: 1px solid $border;
  border-radius: 8px;
  box-shadow: 0px 2px 25px rgba(0, 0, 0, 0.2);

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid $border;

    .title {
      font-size: 18px;
      font-weight: $semi-bold;
    }
    .mark-read {
      font-size: 12px;
      text-decoration: underline;
      color: $gray700;
      cursor: pointer;
    }
  }

  .content {
    height: 300px;
    background: $gray200;
    overflow-y: scroll;
    overscroll-behavior: contain;
    &::-webkit-scrollbar {
      width: 8px;
      background: $surface;
    }
    &::-webkit-scrollbar-thumb {
      background: $gray300;
      border-radius: 56px;
    }
    .item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 16px;
      font-size: 14px;
      padding: 16px 24px;
      cursor: pointer;
      z-index: 100;

      &:hover {
        background: $gray150;
      }
      &:last-child {
        margin-bottom: 8px;
      }

      .icon {
        width: 20px;
        height: 20px;
        stroke: $primary;
      }

      .wrap {
        flex-grow: 2;
      }

      .addr {
        font-size: 14px;
        color: $gray700;
      }
      .timestamp {
        font-size: 12px;
        color: $gray900;
      }
    }
    .empty-state {
      width: 100%;
      height: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 24px;
      font-size: 14px;
      color: $gray700;
    }
  }
}
</style>
