<template>
  <header>
    <div class="wrap">
      <router-link :to="{ path: '/' }">
        <div className="logo"></div>
      </router-link>

      <Nav />

      <button class="btn-wallet" @click="initWallet()">
        <p v-if="walletAddress.length > 0">{{ formatAddress(walletAddress) }}</p>
        <p v-if="walletAddress.length === 0">connect</p>
        <img src="@/assets/ic_wallet.svg" alt="Connect your wallet" />
      </button>
    </div>
  </header>
</template>

<script>
import { defineComponent } from 'vue'
import { formatAddress, formatAmount } from '@/shared'
import { mapActions, mapGetters } from 'vuex'
import Nav from './Nav.vue'

export default defineComponent({
  name: 'Header',
  components: {
    Nav,
  },
  methods: {
    ...mapActions(['initWallet']),
    formatAmount,
    formatAddress,
  },
  computed: mapGetters(['walletAddress']),
})
</script>

<style lang="scss" scoped>
@import '../style/header.scss';
</style>
