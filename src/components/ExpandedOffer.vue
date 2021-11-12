<template>
  <li class="expanded" :key="`${offer.id}-expanded`">
    <div class="owner">
      <p class="wallet">{{ formatAddress(offer.owner) }}</p>
      <p class="n-trades">352 trades</p>
    </div>

    <div class="form-separator"></div>

    <form action="">
      <div class="input">
        <label for="buy">I want to pay</label>
        <CurrencyInput
          v-model="fiatAmount"
          @focus="watchingReceive === false && watchingBuy === true"
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
          ref="buyAmountInput"
        />
      </div>
      <div class="input">
        <label for="sell">I want to receive</label>
        <CurrencyInput
          v-model="cryptoAmount"
          :placeholder="this.cryptoPlaceholder"
          @focus="watchingReceive === true && watchingBuy === false"
          :options="{
            currency: 'UST',
            currencyDisplay: 'code',
            hideCurrencySymbolOnFocus: false,
            hideGroupingSeparatorOnFocus: false,
            precision: 2,
          }"
          ref="receiveAmountInput"
        />
        <p>
          Min ${{ formatAmount(offer.min_amount) }} - Max ${{
            formatAmount(offer.max_amount)
          }}
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
          <p class="margin">0% above market</p>
          <p class="value">
            1 UST = {{ offer.fiat_currency.toUpperCase() }}
            {{ cryptoFiatPrice.toFixed(2) }}
          </p>
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
        <button class="secondary" @click="expandedOffer = null">cancel</button>
        <button class="primary" @click="newTrade()">open transaction</button>
      </div>
    </div>
  </li>
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import CurrencyInput from "./CurrencyInput.vue";
import { formatAddress, formatAmount } from "@/shared";

export default defineComponent({
  name: "ExpandedOffer",
  props: ["offer"],
  components: {
    CurrencyInput,
  },
  data() {
    return {
      cryptoAmount: NaN,
      fiatAmount: NaN,
      tradingFee: 0.0,
      watchingReceive: false,
      watchingBuy: false,
      secondsUntilRateRefresh: 0,
      refreshRateInterval: -1,
    };
  },
  mounted() {
    this.startExchangeRateRefreshTimer();
  },
  unmounted: function() {
    clearInterval(this.refreshRateInterval);
  },
  methods: {
    ...mapActions(["fetchUsdRates", "openTrade"]),
    newTrade: function() {
      console.log("create trade with: ", this.offer.id, this.offer.min_amount);
      this.openTrade({ offerId: this.offer.id, ustAmount: this.cryptoAmount });
    },
    focus: function() {
      let buyInput = this.$refs.buyAmountInput;
      buyInput.focus();
      //this.scrollToElement(buyInput)
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
      return `${symbol} ${min} - ${symbol} ${max}`;
    },
    minMaxCryptoStr: function() {
      let symbol = "UST"; //TODO: get from offer
      let min = (parseInt(this.offer.min_amount) / 1000000).toFixed(2);
      let max = (parseInt(this.offer.max_amount) / 1000000).toFixed(2);
      return `${symbol} ${min} - ${symbol} ${max}`;
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
      if (this.watchingBuy) {
        let fiatCurrency = this.offer.fiat_currency.toUpperCase();
        let usdRate = this.getUsdRate(fiatCurrency);
        let cryptoAmount = parseFloat(val) / usdRate;
        this.tradingFee = cryptoAmount * 0.01;
        this.$nextTick(() => {
          this.$refs.receiveAmountInput.update(cryptoAmount - this.tradingFee);
        });
      }
    },
    cryptoAmount: function(val) {
      if (this.watchingReceive) {
        let fiatCurrency = this.offer.fiat_currency.toUpperCase();
        let usdRate = this.getUsdRate(fiatCurrency);
        this.tradingFee = parseFloat(val) * 0.01;
        this.$nextTick(() => {
          let fiatAmount =
            parseFloat(val) * usdRate + this.tradingFee * usdRate;
          this.$refs.buyAmountInput.update(fiatAmount);
        });
      }
    },
  },
});
</script>

<style lang="scss" scoped>
@import "../style/tokens.scss";

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

      .primary {
        background-color: $gray300;
        color: $primary;
        border: none;
        font-family: inherit;
        font-weight: 700;
        font-size: 16px;
        text-transform: lowercase;
        padding: 8px 24px;
      }

      .secondary {
        background-color: $surface;
        color: $primary;
        border: none;
        font-family: inherit;
        font-weight: 700;
        font-size: 16px;
        text-transform: lowercase;
        padding: 8px 24px;
      }
    }
  }
}
</style>
