<script setup lang="ts">
import { onUnmounted } from 'vue-demi'
import { useClientStore } from '~/stores/client'
import { formatAddress } from '~/shared'
import useNotificationSystem from '~/notification/Notification'

const notification = useNotificationSystem()
const notificationCount = notification.badgeCount

const client = useClientStore()
const userWallet = computed(() => client.userWallet)

function connectWallet() {
  nextTick(async () => {
    await client.connectWallet()
    await notification.register()
  })
}

onUnmounted(() => {
  notification.unregister()
})
</script>

<template>
  <button class="wallet" @click="connectWallet()">
    <p v-if="userWallet.isConnected">
      {{ formatAddress(userWallet.address) }}
      <span> - {{ notificationCount }}</span>
    </p>
    <p v-else>
      connect
    </p>
    <img src="../../assets/ic_wallet.svg" alt="Connect your wallet">
  </button>
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

button {
  @media only screen and (max-width: $mobile) {
    margin: 16px 16px 0 0;
  }
}
</style>
