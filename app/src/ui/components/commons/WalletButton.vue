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
  nextTick(async () => {
    if (!client.applicationConnected) {
      await client.connectWallet()
    } else {
      await client.disconnectWallet()
    }
  })
}

const walletWidget = ref()
function toggleWalletWidget() {
  walletWidget.value.toggleWidget()
}

onUnmounted(() => {
  notification.unregister()
})
</script>

<template>
  <div class="wrap-wallet">
    <button v-if="userWallet.isConnected" class="wallet" @click="toggleWalletWidget()">
      <p>
        {{ formatAddress(userWallet.address) }}
      </p>
      <img src="../../assets/ic_wallet.svg" alt="Connect your wallet" />
    </button>
    <button v-else class="wallet" @click="connectWallet()">
      <p>connect</p>
      <img src="../../assets/ic_wallet.svg" alt="Connect your wallet" />
    </button>
    <WalletWidget v-if="userWallet.isConnected" ref="walletWidget" @connectWidget="connectWallet" />
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
