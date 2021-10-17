<template>
  <section class="create-trade">
    <h1 v-if="offer.offer_type == 'sell'">
      You are buying from <span>{{ formatAddress(offer.owner) }}</span>
    </h1>
    <h1 v-if="offer.offer_type == 'buy'">
      You are selling to <span>{{ formatAddress(offer.owner) }}</span>
    </h1>
    <h2>1 UST = COP$ {{ formatAmount(getUsdRate(offer.fiat_currency), false) }}</h2>
    <h2>Min ${{ formatAmount(offer.min_amount) }} Max ${{ formatAmount(offer.max_amount) }}</h2>

    <div className="input-wrap">
      <img src="@/assets/ic_ust.svg" alt="UST" />
      <span>UST</span>
      <input type="text" v-model="ustAmount" @keyup="ustAmountChanged" />
    </div>
    <div className="input-wrap">
      <img src="@/assets/co.svg" alt="" />
      <span>COP</span>
      <input type="text" v-model="copAmount" @keyup="copAmountChanged" />
    </div>

    <img class="icon-separator" src="@/assets/ic_arrow.svg" alt="" />

    <div className="receipt">
      <div className="row">
        <p>Trading fee</p>
        <p>{{ tradingFee }} COP</p>
      </div>
      <div className="row">
        <p v-if="offer.offer_type == 'buy'">You will send</p>
        <p v-if="offer.offer_type == 'sell'">You will get</p>
        <p className="bold">{{ finalAmount }} UST</p>
      </div>
      <div className="row">
        <p v-if="offer.offer_type == 'buy'">You will receive</p>
        <p v-if="offer.offer_type == 'sell'">You will pay</p>
        <p className="bold color">{{ copAmount }} COP</p>
      </div>
    </div>

    <button v-if="loading" disabled>opening trade...</button>
    <button v-else @click="openTrade({ offerId: offer.id, ustAmount })" :disabled="!valid">
      open trade
    </button>
  </section>
</template>

<script>
import { defineComponent } from 'vue'
import { mapActions, mapGetters } from 'vuex'
import { formatAddress, formatAmount } from '@/shared'

export default defineComponent({
  name: 'Offer',
  data() {
    return {
      ustAmount: 0,
      copAmount: 0,
      loading: false,
      offer: {},
    }
  },
  methods: {
    ...mapActions(['openTrade']),
    formatAddress,
    formatAmount,
    ustAmountChanged() {
      this.copAmount = this.ustAmount * this.getUsdRate(this.offer.fiat_currency)
    },
    copAmountChanged() {
      this.ustAmount = this.copAmount / this.getUsdRate(this.offer.fiat_currency)
    },
  },
  created() {
    this.offer = this.getOfferById(this.$route.params.id)
  },
  computed: {
    ...mapGetters(['getOfferById', 'getUsdRate']),
    valid() {
      const minAmount = this.offer.min_amount / 1000000
      const maxAmount = this.offer.max_amount / 1000000

      const valid = this.ustAmount >= minAmount && this.ustAmount <= maxAmount
      return valid
    },
    tradingFee() {
      return this.copAmount * 0.01
    },
    finalAmount() {
      return this.ustAmount * 0.99
    },
  },
})
</script>
