<template>
    <main>
        <h3>Open Trades</h3>
        <!-- Open Trades section-->
        <section v-if="hasOpenTrades">
            <TradeOpenItem
                v-for="tradeInfo in openTrades"
                :tradeAddr="tradeInfo.trade.addr"
                :key="tradeInfo.trade.addr"
            />
        </section>
        <section v-else class="card">
            <p>Nothing here yet</p>
        </section>
        <!-- End Open Trades section-->
        <!-- Trades History section-->
        <h3 v-if="hasClosedTrades">Trade History</h3>
        <section v-if="hasClosedTrades" class="trade-history-table card">
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
        <!-- End Trades History section -->
    </main>
</template>

<script>
import { defineComponent } from "vue";
import TradeOpenItem from "../components/trades/TradeOpenItem.vue";
import TradeHistoryItem from "../components/trades/TradeHistoryItem.vue";
import { mapActions, mapGetters } from "vuex";
import { onSnapshot, doc } from "firebase/firestore";
import { db } from "../store/firebase";

export default defineComponent({
    name: "Trades",
    components: {
        TradeOpenItem,
        TradeHistoryItem,
    },
    methods: {
        ...mapActions(["fetchTradeInfos"]),
    },
    computed: {
        ...mapGetters(["walletAddress", "trades"]),
        openTrades: function() {
            return this.trades.filter(
                // TODO create a open states list
                (tradeInfo) =>
                    !tradeInfo.expired &&
                    [
                        "request_created",
                        "request_accepted",
                        "escrow_funded",
                        "fiat_deposited",
                        "escrow_disputed",
                    ].indexOf(tradeInfo.trade.state) >= 0,
            );
        },
        hasOpenTrades: function() {
            return this.openTrades.length > 0;
        },
        closedTrades: function() {
            return this.trades.filter(
                // TODO create a closed states list
                (tradeInfo) =>
                    tradeInfo.expired ||
                    [
                        "request_canceled",
                        "request_expired",
                        "escrow_refunded",
                        "escrow_released",
                        "settled_for_maker",
                        "settled_for_taker",
                    ].indexOf(tradeInfo.trade.state) >= 0,
            );
        },
        hasClosedTrades: function() {
            return this.closedTrades.length > 0;
        },
    },
    mounted: function() {
        scrollTo(0, 0);
        if (this.walletAddress) {
            onSnapshot(doc(db, "tradeRequests", this.walletAddress), () => {
                this.fetchTradeInfos();
            });
        }
        this.refreshInterval = setInterval(() => {
            this.fetchTradeInfos();
        }, 5000);
    },
    unmounted: function() {
        clearInterval(this.refreshInterval);
    },
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
    width: 17.5%;
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
    width: 20%;
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
