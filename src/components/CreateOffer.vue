<template>
  <select v-model="offerType">
    <option value="0">Buy</option>
    <option value="1">Sell</option>
  </select>
  <br/>
  <label>Min amount:</label>
  <input type="text" v-model="minAmount" />
  <br/>
  <label>Max amount:</label>
  <input type="text" v-model="maxAmount" />
  <br/>
  <button @click="createOffer(minAmount, maxAmount)">Create</button>
</template>

<script>
import { defineComponent } from 'vue'
import { formatAddress, formatAmount } from '@/shared'
import { mapActions, mapGetters } from 'vuex'

export default defineComponent({
  name: 'CreateOffer',
  data() {
    return {
      minAmount: 10000000,
      maxAmount: 500000000,
      offerType: 1
    }
  },
  methods: {
    ...mapActions(['initWallet', 'newOffer']),
    formatAmount,
    formatAddress,
    createOffer(min_amount, max_amount) {
      let offerType = parseInt(this.offerType) === 0 ? 'buy' : 'sell'
      const newOffer = {
        create: {
          offer: {
            offer_type: offerType,
            fiat_currency: 'BRL',
            min_amount,
            max_amount,
          },
        },
      }
      this.newOffer({ offer: newOffer })
    },
  },
  computed: mapGetters(['walletAddress']),
  created() {
    this.initWallet()
  },
})
</script>

<style lang="scss" scoped></style>
