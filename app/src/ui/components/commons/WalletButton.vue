<script setup lang="ts">
import { onUnmounted } from 'vue-demi'
import WalletWidget from './WalletWidget.vue'
import { useClientStore } from '~/stores/client'
import { formatAddress } from '~/shared'
import useNotificationSystem from '~/notification/Notification'
import { WalletEvents, trackWalletConnection } from '~/analytics/analytics'

const notification = useNotificationSystem()
const client = useClientStore()
const userWallet = computed(() => client.userWallet)

watch(userWallet, async (wallet) => {
  if (wallet.isConnected) {
    await notification.register()
    trackWalletConnection(WalletEvents.connected, wallet.address)
  } else {
    await notification.unregister()
    trackWalletConnection(WalletEvents.disconnected)
  }
})

function connectWallet() {
  nextTick(async () => await client.connectWallet())
}

function disconnectWallet() {
  nextTick(async () => await client.disconnectWallet())
}

const walletWidget = ref()
function toggleWalletWidget() {
  if (userWallet.value.isConnected) {
    walletWidget.value.toggleWidget()
  } else {
    connectWallet()
  }
}

onUnmounted(() => {
  notification.unregister()
})
</script>

<template>
  <div class="wrap-wallet">
    <button class="wallet" @click="toggleWalletWidget()">
      <slot v-if="userWallet.isConnected">
        <p>
          {{ formatAddress(userWallet.address) }}
        </p>
        <img src="../../assets/ic_wallet.svg" alt="Connect your wallet" />
      </slot>
      <slot v-else>
        <p>connect</p>
        <img src="../../assets/ic_wallet.svg" alt="Connect your wallet" />
      </slot>
    </button>
    <WalletWidget v-if="userWallet.isConnected" ref="walletWidget" @disconnect="disconnectWallet()" />
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.wrap-wallet {
  position: relative;
}

button {
  &:hover {
    color: $primary;
  }
  @media only screen and (max-width: $mobile) {
    margin: 16px 16px 0 0;
  }
}
</style>
