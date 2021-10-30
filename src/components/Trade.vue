<template>
  <section class="create-trade">
    <h1 v-if="trade.offer.offer_type == 'buy'">
      You are selling <img src="@/assets/ic_ust.svg" />
      <b>{{ formatAmount(trade.amount) }} UST </b> to
      <span>{{ formatAddress(trade.offer.owner) }}</span> <br />for <img src="@/assets/co.svg" /><b>
        {{ formatAmount(tradeAmountInFiat) }} COP</b
      >
    </h1>

    <h1 v-if="isBuying">
      You are buying {{ trade.amount }} from <span>{{ formatAddress(trade.offer.owner) }}</span>
    </h1>
    <h1 v-if="isSelling">
      You are selling {{ this.trade.amount }} to
      <span>{{ formatAddress(this.trade.recipient) }}</span>
    </h1>

    <div className="receipt">
      <div className="row">
        <p>Seller puts UST in escrow</p>
        <p className="color" v-if="isFundedOrClosed">Done</p>
        <p v-else>Waiting</p>
      </div>
      <div className="row">
        <p>Escrow released to buyer</p>
        <p className="color" v-if="trade.state == 'closed'">Done</p>
        <p v-else>Waiting</p>
      </div>
    </div>

    <!--Funding Escrow-->
    <button v-if="isFundingEscrow" disabled>funding escrow...</button>
    <button @click="fundEscrow" v-else-if="shouldFundEscrow">fund escrow</button>

    <!-- Releasing Escrow -->
    <button disabled v-if="isReleasingEscrow">releasing escrow...</button>
    <button
      @click="releaseEscrow({ tradeAddress: $route.params.id })"
      v-else-if="shouldReleaseEscrow"
    >
      release escrow
    </button>

    <h2 v-if="isWaitingForEscrow">Waiting for escrow</h2>

    <h2 v-if="trade.state === 'expired'">Trade expired</h2>
    <h2 v-if="trade.state === 'closed'">Trade successful</h2>
  </section>
</template>

<script>
import { defineComponent } from 'vue'
import { mapActions, mapGetters } from 'vuex'
import { formatAddress, formatAmount } from '@/shared'

export default defineComponent({
  name: 'Trade',
  data() {
    return {
      loading: false,
    }
  },
  async created() {
    if (this.trade == null) {
      this.loading = true
      this.trade = await this.fetchTrade({ tradeAddress: this.$route.params.id })
      if (!this.getUsdRate(this.trade.offer.fiat_currency)) {
        await this.fetchUsdRates()
      }
      this.loading = false
    }
  },
  computed: {
    ...mapGetters(['trades', 'getTradeById', 'walletAddress', 'getUsdRate']),
    shouldFundEscrow() {
      return (
        this.trade.offer.offer_type == 'sell' &&
        this.trade.state == 'created' &&
        this.trade.sender == this.walletAddress
      )
    },
    isFundingEscrow() {
      return this.shouldFundEscrow && this.loading
    },
    shouldReleaseEscrow() {
      return this.trade.state == 'escrow_funded' && this.trade.sender == this.walletAddress
    },
    isReleasingEscrow() {
      return this.shouldReleaseEscrow && this.loading
    },
    isWaitingForEscrow() {
      return (
        this.trade.offer.offer_type == 'buy' &&
        !this.loading &&
        this.trade.state != 'closed' &&
        this.trade.sender != this.walletAddress
      )
    },
    isBuying() {
      return this.trade.offer.offer_type == 'sell' && this.trade.offer.owner == this.walletAddress
    },
    isSelling() {
      return this.trade.offer.offer_type == 'sell' && this.trade.offer.owner == this.walletAddress
    },
    isFundedOrClosed() {
      return true
    },
    tradeAmountInFiat() {
      return this.getUsdRate(this.trade.offer.fiat_currency) * this.trade.amount
    },
    trade() {
      return this.getTradeById(this.$route.params.id)
    },
  },
  methods: {
    ...mapActions(['fetchTrade', 'fetchUsdRates', 'releaseEscrow']),
    formatAddress,
    formatAmount,
  },
})
</script>
