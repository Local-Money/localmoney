<template>
  <main>
    <h3>Open Trades</h3>
    <TradeOpenItem
      v-for="tradeInfo in openTrades"
      :tradeAddr="tradeInfo.trade.addr"
      :key="tradeInfo.trade.addr"
    />
    <h3>Trade History</h3>
    <section class="trade-history-table card">
      <div class="table-header">
        <div class="col-1"><p>Date</p></div>
        <div class="col-2"><p>Type</p></div>
        <div class="col-3"><p>Crypto</p></div>
        <div class="col-4"><p>Amount</p></div>
        <div class="col-5"><p>Trader</p></div>
        <div class="col-6"><p>Status</p></div>
      </div>
      <TradeHistoryItem
        v-for="tradeInfo in closedTrades"
        :tradeAddr="tradeInfo.trade.addr"
        :key="tradeInfo.trade.addr"
      />
    </section>
  </main>
</template>

<script>
import { defineComponent } from "vue";
import TradeOpenItem from "../components/trades/TradeOpenItem.vue";
import TradeHistoryItem from "../components/trades/TradeHistoryItem.vue";
import {mapActions, mapGetters} from "vuex";
import { onSnapshot, doc } from "firebase/firestore"
import { db } from "../store/firebase";

export default defineComponent({
  name: "Trades",
  components: {
    TradeOpenItem,
    TradeHistoryItem,
  },
  methods: {
    ...mapActions(["fetchTradeInfos"])
  },
  computed: {
    ...mapGetters(["walletAddress", "trades"]),
    openTrades: function() {
      return this.trades.filter(
        (tradeInfo) => !tradeInfo.expired &&
          ["created", "escrow_funded"].indexOf(tradeInfo.trade.state) >= 0
      );
    },
    closedTrades: function() {
      return this.trades.filter(
        (tradeInfo) => tradeInfo.expired ||
          ["created", "escrow_funded"].indexOf(tradeInfo.trade.state) === -1
      );
    },
  },
  mounted: function () {
    scrollTo(0, 0)
    if (this.walletAddress) {
      onSnapshot(doc(db, 'tradeRequests', this.walletAddress), () => {
        this.fetchTradeInfos()
      })
    }
  }
});
</script>

<style lang="scss" scoped>
@import "../style/pages.scss";

h3 {
  margin: 32px 0;
  font-size: 18px;
  font-weight: $semi-bold;
}

.trade-history-table {
  .table-header {
    display: flex;
    flex-direction: row;
    border-bottom: 1px solid $border;
    padding: 16px;
    margin-bottom: 16px;

    p {
      font-size: 14px;
      font-weight: $semi-bold;
      color: $gray700;
    }
  }
}

.col-1,
:deep(.col-1) {
  width: 12.5%;
}

.col-2,
:deep(.col-2) {
  width: 12.5%;
}

.col-3,
:deep(.col-3) {
  width: 20%;
}

.col-4,
:deep(.col-4) {
  width: 25%;
}

.col-5,
:deep(.col-5) {
  width: 20%;
}

.col-6,
:deep(.col-6) {
  width: 10%;
}
</style>
