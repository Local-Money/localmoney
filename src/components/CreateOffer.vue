<template>
  <input type="text" v-bind="minAmount" />
  <input type="text" v-bind="maxAmount" />
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
    }
  },
  methods: {
    ...mapActions(['initWallet', 'newOffer']),
    formatAmount,
    formatAddress,
    createOffer(min_amount, max_amount) {
      const newOffer = {
        create: {
          offer: {
            offer_type: 'buy',
            fiat_currency: 'BRL',
            min_amount,
            max_amount,
          },
        },
      }
      console.log('Create offer', newOffer)
      this.newOffer({ offer: newOffer })
      //this.actions.createOffer(newOffer)
    },
  },
  computed: mapGetters(['walletAddress']),
  created() {
    this.initWallet()
  },
})
</script>

<style lang="scss" scoped></style>
