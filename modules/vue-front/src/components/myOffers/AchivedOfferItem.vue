<template>
    <div class="wrap-table-item">
        <div class="col-1">
            <p>{{ type }}</p>
        </div>
        <div class="col-2">
            <p>{{ currentDate }}</p>
        </div>
        <div class="col-3">
            <p>{{ fiatCurrency }}</p>
        </div>
        <div class="col-4">
            <p>{{ limit }}</p>
        </div>
        <div class="col-5">
            <p>{{ price }}</p>
        </div>
        <div class="col-6 unarchive">
            <p @click="unarchive()">Unarchive</p>
        </div>
    </div>
</template>

<script>
import { defineComponent } from "vue";
import { formatAmount, formatFiatAmount, formatDate } from "@/shared";
import { mapActions, mapGetters } from "vuex";

export default defineComponent({
    name: "ArchivedOfferItem",
    props: ["offer"],
    methods: {
        ...mapActions(["fetchUsdRates", "unarchiveOffer"]),
        formatAmount,
        formatDate,
        unarchive: function() {
            this.unarchiveOffer(this.offer);
        },
    },
    mounted() {
        this.fetchUsdRates();
    },
    computed: {
        ...mapGetters(["getUsdRate"]),
        currentDate: function() {
            let date = new Date(this.offer.timestamp * 1000);
            return this.formatDate(date, false);
        },
        fiatCurrency: function() {
            return this.offer.fiat_currency;
        },
        price: function() {
            const usdRate = this.getUsdRate(this.offer.fiat_currency);
            const price = this.offer.rate * usdRate;
            return `${this.offer.fiat_currency} ${formatFiatAmount(price)}`;
        },
        limit: function() {
            const min = formatAmount(this.offer.min_amount);
            const max = formatAmount(this.offer.max_amount);
            return `$${min} - $${max}`;
        },
        type: function() {
            return this.offer.offer_type === "buy" ? "Buying" : "Selling";
        },
    },
});
</script>

<style lang="scss" scoped>
@import "../../style/tokens";

.wrap-table-item {
    display: flex;
    flex-direction: row;
    padding: 16px;

    p {
        font-size: 14px;
        font-weight: $regular;
    }

    .unarchive {
        cursor: pointer;
        color: $primary;

        p {
            font-weight: 600;
        }
    }
}
</style>
