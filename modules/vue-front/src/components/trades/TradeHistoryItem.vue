<template>
  <div class="wrap-table-item">
    <div class="col-1">
      <p>{{ currentDate }}</p>
    </div>
    <div class="col-2">
      <p>{{ tradeType }}</p>
    </div>
    <div class="col-3">
      <p>{{ formatAmount(this.tradeInfo.trade.ust_amount) }} UST</p>
    </div>
    <div class="col-4">
      <p>{{ fiatAmountStr }}</p>
    </div>
    <div class="col-5 trader">
      <p>{{ formatAddress(counterparty) }}</p>
    </div>
    <div class="col-6">
      <p>{{ tradeState(this.tradeInfo.trade.state) }}</p>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import {
  formatAddress,
  formatAmount,
  formatTradeState,
  formatDate,
} from "@/shared";
import { mapGetters } from "vuex";

export default defineComponent({
  name: "TradeHistoryItem",
  props: ["tradeAddr"],
  methods: {
    formatAmount,
    formatAddress,
    formatTradeState,
    formatDate,
    tradeState: function(state) {
      if (state == "created") {
        return "Expired";
      } else {
        return formatTradeState(state);
      }
    },
  },
  computed: {
    ...mapGetters(["walletAddress", "getUsdRate", "getTradeInfo"]),
    tradeInfo: function() {
      return this.getTradeInfo(this.$props.tradeAddr);
    },
    currentDate: function() {
      let date = new Date();
      return this.formatDate(date);
    },
    fiatCurrency: function() {
      return this.tradeInfo.offer.fiat_currency;
    },
    fiatAmountStr: function() {
      const fiatAmount = formatAmount(
        (this.tradeInfo.trade.ust_amount / 1000000) *
          this.getUsdRate(this.fiatCurrency),
        false
      );
      return `${this.fiatCurrency} ${fiatAmount}`;
    },
    tradeType: function() {
      return this.tradeInfo.trade.recipient === this.walletAddress
        ? "Buying"
        : "Selling";
    },
    counterparty: function() {
      const trade = this.tradeInfo.trade;
      return this.walletAddress === trade.sender
        ? trade.recipient
        : trade.sender;
    },
  },
});
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
