<template>
  <div class="wrap-table-item">
    <div class="col-1"><p>18 Jul 2021</p></div>
    <div class="col-2"><p>Selling</p></div>
    <div class="col-3"><p>{{ formatAmount(trade.ust_amount) }} UST</p></div>
    <div class="col-4"><p>$365,900.00 COP</p></div>
    <div class="col-5 trader"><p>{{ formatAddress(counterparty) }}</p></div>
    <div class="col-6"><p>{{ formatTradeState(trade.state) }}</p></div>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {formatAddress, formatAmount, formatTradeState } from "@/shared";
import {mapGetters} from "vuex";

export default defineComponent({
  name: 'TradeHistoryItem',
  props: ['trade'],
  methods: {
    formatAmount,
    formatAddress,
    formatTradeState
  },
  computed: {
    ...mapGetters(['walletAddress']),
    counterparty: function () {
      return this.walletAddress === this.trade.sender ? this.trade.recipient : this.trade.sender;
    }
  }
})
</script>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.wrap-table-item {
  display: flex;
  flex-direction: row;
  padding: 16px;

  p {
    font-size: 14px;
    font-weight: $regular;
  }

  .trader {
    color: $primary;
  }
}
</style>
