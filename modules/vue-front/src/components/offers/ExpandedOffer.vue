<template>
    <div class="expanded" :key="`${offer.id}-expanded`" ref="expandedCard">
        <div class="owner">
            <p class="wallet">{{ formatAddress(offer.owner) }}</p>
            <p class="n-trades">0 trades</p>
        </div>

        <div class="form-separator"></div>

        <form action="">
            <div class="input">
                <label for="from">{{ fromLabel() }}</label>
                <CurrencyInput
                    @focus="
                        watchingCrypto = true;
                        watchingFiat = false;
                    "
                    v-model="cryptoAmount"
                    :placeholder="this.cryptoPlaceholder"
                    :options="{
                        currency: 'UST',
                        currencyDisplay: 'code',
                        hideCurrencySymbolOnFocus: false,
                        hideGroupingSeparatorOnFocus: false,
                        precision: 2,
                        valueRange: {
                            min: minAmountInCrypto(offer),
                            max: maxAmountInCrypto(offer),
                        },
                    }"
                    ref="cryptoAmountInput"
                />
                <p>
                    Limit
                    <b @click="useMinCrypto()">{{ minMaxCryptoStr[0] }}</b> -
                    <b @click="useMaxCrypto()">{{ minMaxCryptoStr[1] }}</b>
                </p>
            </div>

            <div class="input">
                <label for="to">{{ toLabel() }}</label>
                <CurrencyInput
                    v-model="fiatAmount"
                    @focus="
                        watchingCrypto = false;
                        watchingFiat = true;
                    "
                    :placeholder="this.fiatPlaceholder"
                    :options="{
                        currency: offer.fiat_currency.toUpperCase(),
                        currencyDisplay: 'code',
                        hideCurrencySymbolOnFocus: false,
                        hideGroupingSeparatorOnFocus: false,
                        valueRange: {
                            min: minAmountInFiat(offer),
                            max: maxAmountInFiat(offer),
                        },
                    }"
                    ref="fiatAmountInput"
                />
                <p>
                    Limit <b @click="useMinFiat()">{{ minMaxFiatStr[0] }}</b> -
                    <b @click="useMaxFiat()">{{ minMaxFiatStr[1] }}</b>
                </p>
            </div>
        </form>

        <div class="receipt">
            <div class="price">
                <p class="label">Price</p>
                <div class="wrap">
                    <p class="ticker">
                        Will refresh in {{ this.secondsUntilRateRefresh }}s
                    </p>
                    <p class="margin">
                        {{ marginRate.marginOffset }}%
                        {{ marginRate.margin }} market
                    </p>
                    <p class="value">1 UST = {{ offerPrice }}</p>
                </div>
            </div>

            <div class="summary">
                <div class="wrap">
                    <div class="item">
                        <p class="info">Trading Fee</p>
                        <p>UST {{ tradingFee.toFixed(2) }}</p>
                    </div>
                </div>
            </div>

            <div class="wrap-btns">
                <button class="secondary" @click="$emit('cancel', offer)">
                    cancel
                </button>
                <button class="primary" @click="newTrade()" :disabled="!valid">
                    open trade
                </button>
            </div>
        </div>
    </div>
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import CurrencyInput from "../CurrencyInput.vue";
import {
    calculateFiatPriceByRate,
    convertOfferRateToMarginRate,
    formatAddress,
    formatAmount,
    scrollToElement,
} from "@/shared";

export default defineComponent({
    name: "ExpandedOffer",
    props: ["offer"],
    components: {
        CurrencyInput,
    },
    data() {
        return {
            cryptoAmount: 0,
            fiatAmount: 0,
            tradingFee: 0.0,
            watchingCrypto: true,
            watchingFiat: false,
            secondsUntilRateRefresh: 0,
            refreshRateInterval: -1,
            marginRate: convertOfferRateToMarginRate(this.offer.rate),
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
        ...mapActions(["fetchUsdRates", "openTrade"]),
        scrollToElement,
        newTrade: function() {
            this.openTrade({ offer: this.offer, ustAmount: this.cryptoAmount });
        },
        isOfferTypeBuy: function() {
            return this.offer.offer_type === "buy";
        },
        fromLabel: function() {
            return this.isOfferTypeBuy() ? "I want to sell" : "I want to buy";
        },
        toLabel: function() {
            return this.isOfferTypeBuy() ? "I will receive" : "I will pay";
        },
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
        minAmountInCrypto: function() {
            return parseInt(this.offer.min_amount) / 1000000;
        },
        maxAmountInCrypto: function() {
            return parseInt(this.offer.max_amount) / 1000000;
        },
        minAmountInFiat: function() {
            let usdRate = this.fiatPriceByRate;
            return usdRate * (parseInt(this.offer.min_amount) / 1000000);
        },
        maxAmountInFiat: function() {
            let usdRate = this.fiatPriceByRate;
            return usdRate * (parseInt(this.offer.max_amount) / 1000000);
        },
        useMinCrypto: function() {
            this.watchingFiat = false;
            this.watchingCrypto = true;
            this.$refs.cryptoAmountInput.update(this.minAmountInCrypto());
        },
        useMaxCrypto: function() {
            this.watchingFiat = false;
            this.watchingCrypto = true;
            this.cryptoAmount = this.maxAmountInCrypto();
            this.$refs.cryptoAmountInput.update(this.maxAmountInCrypto());
        },
        useMinFiat: function() {
            this.watchingCrypto = false;
            this.watchingFiat = true;
            this.$refs.fiatAmountInput.update(this.minAmountInFiat());
        },
        useMaxFiat: function() {
            this.watchingCrypto = false;
            this.watchingFiat = true;
            this.$refs.fiatAmountInput.update(this.maxAmountInFiat());
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
        usdRate: function() {
            return this.getUsdRate(this.offer.fiat_currency);
        },
        fiatPriceByRate: function() {
            return calculateFiatPriceByRate(this.usdRate, this.offer.rate);
        },
        offerPrice: function() {
            return `${this.offer.fiat_currency} ${formatAmount(
                this.fiatPriceByRate,
                false,
            )}`;
        },
    },
    watch: {
        fiatAmount: function(val) {
            if (this.watchingFiat) {
                let usdRate = this.fiatPriceByRate;
                let cryptoAmount = parseFloat(val) / usdRate;
                this.tradingFee = cryptoAmount * 0.01;
                this.$nextTick(() => {
                    this.$refs.cryptoAmountInput.update(cryptoAmount);
                });
            }
        },
        cryptoAmount: function(val) {
            if (this.watchingCrypto) {
                let usdRate = this.fiatPriceByRate;
                this.tradingFee = parseFloat(val) * 0.01;
                this.$nextTick(() => {
                    let fiatAmount = parseFloat(val) * usdRate;
                    this.$refs.fiatAmountInput.update(fiatAmount);
                });
            }
        },
    },
});
</script>

<style lang="scss" scoped>
@import "@/style/tokens.scss";

.owner {
    grid-column: 1/2;
    grid-row: 1;

    .wallet {
        font-size: 18px;
        font-weight: 600;
        color: $base-text;
    }

    .n-trades {
        font-size: 14px;
        color: $gray600;
    }

    @media only screen and (max-width: 550px) {
        .owner {
            display: inline-flex;
        }
    }
}

.expanded {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    gap: 16px;

    @media only screen and (max-width: 1050px) {
        grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    }

    @media only screen and (max-width: 550px) {
        grid-template-columns: 1fr 1fr;
        padding: 24px 24px;
    }

    .form-separator {
        width: 100%;
        height: 1px;
        background-color: $border;
        margin: 8px 0;
        grid-column: 1/7;
        grid-row: 2;
    }

    form {
        grid-column: 1/3;
        grid-row: 3;
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
        grid-column: 3/7;
        grid-row: 3;
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
