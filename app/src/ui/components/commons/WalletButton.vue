<script setup lang="ts">
import { onUnmounted } from 'vue-demi'
import { useClientStore } from '~/stores/client'
import { formatAddress } from '~/shared'
import useNotificationSystem from '~/notification/Notification'
import { WalletState, trackWalletConnection } from '~/analytics/analytics'

const notification = useNotificationSystem()
const client = useClientStore()
const userWallet = computed(() => client.userWallet)

watch(userWallet, (wallet) => {
  if (wallet.isConnected) {
    trackWalletConnection(WalletState.connected, wallet.address)
  } else {
    trackWalletConnection(WalletState.disconnected)
  }
})

function connectWallet() {
  nextTick(async () => {
    if (!client.applicationConnected) {
      await client.connectWallet()
      await notification.register()
    } else {
      await client.disconnectWallet()
      await notification.unregister()
    }
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
    </p>
    <p v-else>connect</p>
    <img src="../../assets/ic_wallet.svg" alt="Connect your wallet" />
  </button>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

button {
  @media only screen and (max-width: $mobile) {
    margin: 16px 16px 0 0;
  }
}
</style>
