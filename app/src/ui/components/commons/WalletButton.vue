<script setup lang="ts">
import { useClientStore } from '~/stores/client'
import { formatAddress } from '~/shared'

const client = useClientStore()
const userWallet = computed(() => client.userWallet)

function connectWallet() {
  nextTick(async () => {
    await client.connectWallet()
  })
}
</script>

<template>
  <button class="wallet" @click="connectWallet()">
    <p v-if="userWallet.isConnected">
      {{ formatAddress(userWallet.address) }}
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
