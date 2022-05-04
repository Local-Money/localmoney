<template>
    <div class="expanded" :key="`${offer.id}-expanded`" ref="expandedCard">
        <div class="offer-type">
            <p class="type">{{ offer.offer_type }}ing</p>
        </div>

        <div class="horizontal-separator"></div>

        <div class="wrap-status">
            <select name="select-status" class="select-status">
                <option value="">Active</option>
                <option value="">Pause</option>
                <option value="">Archive</option>
            </select>
        </div>

        <div class="price">
            <p class="label">Offer Status</p>
            <div class="buy-sell">
                <button
                    v-on:click="updatedOffer.state = 'active'"
                    v-bind:class="{ focus: updatedOffer.state == 'active' }"
                >
                    Active
                </button>
                <div class="separator"></div>
                <button
                    v-on:click="updatedOffer.state = 'paused'"
                    v-bind:class="{ focus: updatedOffer.state == 'paused' }"
                >
                    Paused
                </button>
            </div>
        </div>

        <form action="">
            <div class="min-max">
                <div class="wrap">
                    <label>Min amount:</label>
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

                    <label>Max amount:</label>
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
            </div>
        </form>

        <div class="receipt">
            <div class="wrap-btns">
                <button class="secondary" @click="$emit('cancel', offer)">
                    close
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
            },
        };
    },
    mounted() {
        this.startExchangeRateRefreshTimer();
        console.log("this.offer :>> ", this.offer);
        console.log("updatedOffer :>> ", this.updatedOffer);
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
    gap: 24px;

    .offer-type {
        display: flex;
        align-items: center;

        .type {
            font-size: 18px;
            font-weight: $semi-bold;
            color: $base-text;
            text-transform: capitalize;
        }
    }

    .horizontal-separator {
        width: 100%;
        height: 1px;
        background-color: $border;
        margin: 8px 0;
    }

    form {
        margin-top: 16px;
        padding-right: 56px;

        .input {
            margin-bottom: 24px;
        }

        label {
            font-size: 14px;
            color: $gray600;
            display: block;
        }

        input {
            color: $base-text;
            background-color: $background;
            text-align: right;
        }

        b {
            cursor: pointer;
            font-weight: 600;
        }

        p {
            font-size: 12px;
            color: $gray600;
            text-align: right;
            margin-top: 8px;
        }
    }

    .receipt {
        margin-top: 16px;

        .price {
            margin-bottom: 24px;

            .label {
                font-size: 14px;
                color: $gray600;
            }

            .wrap {
                width: 100%;
                display: inline-flex;
                justify-content: space-between;
                background-color: $gray150;
                border-radius: 8px;
                padding: 10px 24px;
                margin-top: 8px;
                align-items: center;
                gap: 16px;

                .ticker {
                    font-size: 12px;
                    color: $primary;
                }

                .margin {
                    font-size: 14px;
                    color: $gray600;
                }

                .value {
                    font-size: 16px;
                    font-weight: 600;
                    color: $base-text;
                }
            }
        }

        .summary {
            margin-bottom: 24px;

            .label {
                font-size: 14px;
                color: $gray600;
            }

            .wrap {
                width: 100%;
                display: flex;
                flex-direction: column;
                justify-content: space-between;
                background-color: $gray150;
                border-radius: 8px;
                padding: 16px 24px;
                margin-top: 8px;

                gap: 8px;

                .item {
                    display: inline-flex;
                    justify-content: space-between;

                    .price-get {
                        font-weight: 800;
                    }

                    .price-pay {
                        font-weight: 800;
                        color: $primary;
                    }
                }
            }
        }

        .wrap-btns {
            display: flex;
            justify-content: flex-end;
            gap: 24px;
        }
    }
}
</style>
