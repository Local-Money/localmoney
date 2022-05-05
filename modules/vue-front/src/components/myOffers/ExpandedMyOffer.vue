<template>
    <div class="expanded" :key="`${offer.id}-expanded`" ref="expandedCard">
        <div class="offer-type">
            <div class="inner-wrap">
                <div class="wrap-status">
                    <!-- <p class="label">Offer Status</p> -->
                    <select class="bg-gray100" v-model="updatedOffer.state">
                        <option value="active">Active</option>
                        <option value="paused">Pause</option>
                        <option value="archive">Archive</option>
                    </select>
                </div>
                <p class="type">{{ offer.offer_type }}ing</p>
            </div>
            <div class="price">
                <p class="value">
                    COP 3935.00
                    <!-- {{ offer.fiat_currency }} {{ formatAmount(usdRate, false) }} -->
                </p>
            </div>
        </div>
        <div class="horizontal-separator"></div>
        <div class="form-wrap">
            <form action="">
                <div class="input-wrap">
                    <label class="label">Margin</label>
                    <select name="" class="bg-gray100">
                        <option value="">Above</option>
                        <option value="">Below</option>
                    </select>
                </div>

                <div class="input-wrap">
                    <label class="label">Margin offset</label>
                    <input
                        v-model="updatedOffer.rate"
                        type="text"
                        placeholder="0%"
                    />
                </div>
            </form>

            <form action="">
                <div class="input-wrap">
                    <label class="label">Min amount:</label>
                    <CurrencyInput
                        v-model="updatedOffer.min_amount"
                        :placeholder="this.cryptoPlaceholder"
                        :options="{
                            currency: 'UST',
                            currencyDisplay: 'code',
                            hideCurrencySymbolOnFocus: false,
                            hideGroupingSeparatorOnFocus: false,
                            precision: 2,
                        }"
                        ref="minAmount"
                    />
                </div>

                <div class="input-wrap">
                    <label class="label">Max amount:</label>
                    <CurrencyInput
                        v-model="updatedOffer.max_amount"
                        :placeholder="this.cryptoPlaceholder"
                        :options="{
                            currency: 'UST',
                            currencyDisplay: 'code',
                            hideCurrencySymbolOnFocus: false,
                            hideGroupingSeparatorOnFocus: false,
                            precision: 2,
                        }"
                        ref="maxAmount"
                    />
                </div>
            </form>
            <div class="wrap-btns">
                <button class="secondary" @click="$emit('cancel', offer)">
                    cancel
                </button>
                <button
                    class="primary"
                    @click="updateOffer({ updatedOffer: this.updatedOffer })"
                >
                    update
                </button>
            </div>
        </div>
    </div>
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import CurrencyInput from "../CurrencyInput.vue";
import { formatAddress, formatAmount, scrollToElement } from "@/shared";

export default defineComponent({
    name: "ExpandedMyOffer",
    props: ["offer"],
    components: {
        CurrencyInput,
    },
    data() {
        return {
            cryptoAmount: 0,
            fiatAmount: 0,
            tradingFee: 0.0,
            watchingFiat: true,
            watchingCrypto: false,
            secondsUntilRateRefresh: 0,
            refreshRateInterval: -1,
            updatedOffer: {
                ...this.offer,
                min_amount: formatAmount(this.offer.min_amount),
                max_amount: formatAmount(this.offer.max_amount),
                rate: this.offer.rate,
                state: this.offer.state,
            },
        };
    },
    mounted() {
        this.startExchangeRateRefreshTimer();
        this.$nextTick(() => {
            this.focus();
        });
    },
    unmounted: function() {
        clearInterval(this.refreshRateInterval);
    },
    methods: {
        ...mapActions(["fetchUsdRates", "updateOffer"]),
        scrollToElement,
        focus: function() {
            let card = this.$refs.expandedCard;
            scrollToElement(card);
        },
        startExchangeRateRefreshTimer: function() {
            let seconds = 60;
            let countdownInterval = 1000;
            this.refreshRateInterval = setInterval(() => {
                this.$data.secondsUntilRateRefresh = --seconds;
                if (seconds === 0) {
                    this.refreshExchangeRate();
                    seconds = 60;
                }
            }, countdownInterval);
        },
        refreshExchangeRate: function() {
            this.fetchUsdRates();
        },
        formatAddress,
        formatAmount,
        minAmountInFiat: function() {
            let fiatCurrency = this.offer.fiat_currency.toUpperCase();
            let usdRate = this.getUsdRate(fiatCurrency);
            return usdRate * (parseInt(this.offer.min_amount) / 1000000);
        },
        maxAmountInFiat: function() {
            let fiatCurrency = this.offer.fiat_currency.toUpperCase();
            let usdRate = this.getUsdRate(fiatCurrency);
            return usdRate * (parseInt(this.offer.max_amount) / 1000000);
        },
        useMin: function() {
            this.watchingCrypto = true;
            this.cryptoAmount = parseInt(this.offer.min_amount) / 1000000;
        },
        useMax: function() {
            this.watchingCrypto = true;
            this.cryptoAmount = parseInt(this.offer.max_amount) / 1000000;
        },
    },
    computed: {
        ...mapGetters(["getUsdRate"]),
        receiveTotal: function() {
            let amount = this.cryptoAmount;
            if (amount > 0) {
                return amount.toFixed(2);
            } else {
                return "0";
            }
        },
        minMaxFiatStr: function() {
            let symbol = this.offer.fiat_currency.toUpperCase();
            let min = this.minAmountInFiat(this.offer).toFixed(2);
            let max = this.maxAmountInFiat(this.offer).toFixed(2);
            return [`${symbol} ${min}`, `${symbol} ${max}`];
        },
        minMaxCryptoStr: function() {
            let symbol = "UST"; //TODO: get from offer
            let min = (parseInt(this.offer.min_amount) / 1000000).toFixed(2);
            let max = (parseInt(this.offer.max_amount) / 1000000).toFixed(2);
            return [`${symbol} ${min}`, `${symbol} ${max}`];
        },
        cryptoPlaceholder: function() {
            let symbol = "UST"; //TODO: get from offer
            return `${symbol} ${parseFloat(0).toFixed(2)}`;
        },
        fiatPlaceholder: function() {
            let symbol = this.offer.fiat_currency.toUpperCase();
            return `${symbol} 0`;
        },
        cryptoFiatPrice: function() {
            //TODO: We need price source for other cryptos,
            //we can source the Fiat -> USD rate, then USD -> UST and finally UST -> Crypto.
            let fiatCurrency = this.offer.fiat_currency.toUpperCase();
            let usdRate = this.getUsdRate(fiatCurrency);
            return usdRate;
        },
        valid: function() {
            let total = this.cryptoAmount * 1000000;
            let min_amount = parseInt(this.offer.min_amount);
            let max_amount = parseInt(this.offer.max_amount);
            return total >= min_amount && total <= max_amount;
        },
    },
    watch: {
        fiatAmount: function(val) {
            if (this.watchingFiat) {
                let fiatCurrency = this.offer.fiat_currency.toUpperCase();
                let usdRate = this.getUsdRate(fiatCurrency);
                let cryptoAmount = parseFloat(val) / usdRate;
                this.tradingFee = cryptoAmount * 0.01;
                this.$nextTick(() => {
                    this.$refs.receiveAmountInput.update(cryptoAmount);
                });
            }
        },
        cryptoAmount: function(val) {
            if (this.watchingCrypto) {
                let fiatCurrency = this.offer.fiat_currency.toUpperCase();
                let usdRate = this.getUsdRate(fiatCurrency);
                this.tradingFee = parseFloat(val) * 0.01;
                this.$nextTick(() => {
                    let fiatAmount = parseFloat(val) * usdRate;
                    this.$refs.buyAmountInput.update(fiatAmount);
                });
            }
        },
    },
});
</script>

<style lang="scss" scoped>
@import "@/style/tokens.scss";

.expanded {
    display: flex;
    flex-direction: column;

    .offer-type {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .inner-wrap {
            display: flex;
            justify-content: flex-start;
            align-items: center;
            gap: 32px;

            .type {
                font-size: 18px;
                font-weight: $semi-bold;
                color: $base-text;
                text-transform: capitalize;
            }
        }

        .value {
            font-size: 16px;
            font-weight: 600;
            color: $base-text;
            font-weight: $semi-bold;
        }
    }

    .wrap-btns {
        display: flex;
        justify-content: flex-end;
        gap: 24px;
        margin-top: 32px;
    }

    .horizontal-separator {
        width: 100%;
        height: 1px;
        background-color: $border;
        margin: 32px 0 0px;
    }

    .wrap-status {
        display: flex;
        align-items: center;
        min-width: 150px;

        select {
            color: $primary;
        }
    }

    .label {
        font-size: 14px;
        color: $gray600;
        margin-bottom: 8px;
    }

    .form-wrap {
        display: flex;
        align-items: center;
        gap: 32px;
    }

    form {
        width: 100%;
        margin-top: 16px;
        display: flex;
        gap: 16px;
        padding: 8px 0px;

        .input-wrap {
            display: flex;
            flex-direction: column;
            width: 100%;

            select {
            }
        }

        .input {
        }

        input {
            color: $base-text;
            background-color: $background;
            text-align: right;
        }

        p {
            font-size: 12px;
            color: $gray600;
            text-align: right;
            margin-top: 8px;
        }
    }
}
</style>
