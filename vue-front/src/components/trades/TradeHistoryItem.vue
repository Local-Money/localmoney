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
    formatFiatAmount,
    formatTradeState,
    formatDate,
    calculateFiatPriceByRate,
} from "@/shared";
import { mapActions, mapGetters } from "vuex";

export default defineComponent({
    name: "TradeHistoryItem",
    props: ["tradeAddr"],
    methods: {
        ...mapActions(["fetchUsdRates"]),
        formatAmount,
        formatAddress,
        formatTradeState,
        formatDate,
        tradeState: function(state) {
            return formatTradeState(state);
        },
    },
    mounted() {
        this.fetchUsdRates();
    },
    computed: {
        ...mapGetters(["walletAddress", "getUsdRate", "getTradeInfo"]),
        tradeInfo: function() {
            return this.getTradeInfo(this.$props.tradeAddr);
        },
        currentDate: function() {
            let date = new Date(this.tradeInfo.trade.created_at * 1000);
            return this.formatDate(date);
        },
        fiatCurrency: function() {
            return this.tradeInfo.offer.fiat_currency;
        },
        fiatPriceByRate: function() {
            return calculateFiatPriceByRate(
                this.getUsdRate(this.fiatCurrency),
                this.tradeInfo.offer.rate,
            );
        },
        fiatAmountStr: function() {
            const fiatAmount = formatFiatAmount(
                (this.tradeInfo.trade.ust_amount / 1000000) *
                    this.fiatPriceByRate,
            );
            return `${this.fiatCurrency} ${fiatAmount}`;
        },
        tradeType: function() {
            return this.tradeInfo.offer.offer_type === "buy" ? "Buy" : "Sell";
        },
        counterparty: function() {
            const trade = this.tradeInfo.trade;
            return this.walletAddress === trade.seller
                ? trade.buyer
                : trade.seller;
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
