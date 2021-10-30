<template>
  <section>
    <div class="separator"></div>
    <section class="offers-filter">
      <div class="buy-sell">
        <button class="buy">buy</button>
        <div class="separator"></div>
        <button class="sell">sell</button>
      </div>
      <div class="filter">
        <label for="crypto">Crypto</label>
        <select name="crypto" id="crypto">
          <option value="UST">UST</option>
          <option value="UST">LUNA</option>
          <option value="UST">LOTA</option>
          <option value="UST">LOCAL</option>
        </select>
      </div>
      <div class="filter">
        <label for="currency">Currency (FIAT)</label>
        <select name="currency" id="currency">
          <option value="UST">COP</option>
          <option value="UST">BRL</option>
          <option value="UST">USD</option>
        </select>
      </div>
    </section>

    <section class="offers-list">
      <h3>Buy from this sellers</h3>
      <!-- Offers for -->
      <ul>
        <!-- Collapsed Offer -->
        <li
          class="collapsed"
          v-for="offer in collapsedOffers"
          :key="`${offer.id}-collapsed`"
        >
          <div class="owner">
            <p class="wallet">{{ formatAddress(offer.owner) }}</p>
            <p class="n-trades">352 trades</p>
          </div>

          <div class="info">
            <div class="divider"></div>
            <p class="min-max">
              Min ${{ formatAmount(offer.min_amount) }} - Max ${{
                formatAmount(offer.max_amount)
              }}
            </p>
          </div>

          <div class="price">
            <div class="wrap-value">
              <p class="value">COL$ 348.892,53</p>
              <p class="margin">4% above market</p>
            </div>
            <button type="button" @click="this.expandOfferItem(offer)">
              {{ this.offerTypeLabels[offer.offer_type] }}
            </button>
          </div>
        </li>

        <!-- Expanded Offer Desktop -->
        <ExpandedOffer v-if="expandedOffer" :offer="this.expandedOffer" />
        <!-- Expanded Offer Desktop -->
      </ul>
    </section>
  </section>

  <!-- Expanded Offer Mobile -->
  <div class="expanded-mobile" v-if="false">
    <div class="owner">
      <p class="wallet">terra12242343</p>
      <p class="n-trades">352 trades</p>
    </div>

    <div class="payment-info">
      <p class="note">Nubank, Itaú, C6, Mercado Pago, Inter, Caixa, Bradesco</p>
    </div>

    <form action="">
      <div class="input">
        <label for="buy">I want to buy</label>
        <input type="text" placeholder="100.00" ref="buyAmountInput" />
      </div>
      <div class="input">
        <label for="sell">I will receive</label>
        <input type="text" placeholder="100.00" />
        <p>Min - 1 | Max - 50</p>
      </div>
    </form>

    <div class="receipt">
      <div class="price">
        <div class="wrap-price">
          <p class="label">Price</p>
          <p class="ticker">Will refresh in 47s</p>
        </div>
        <div class="wrap">
          <p class="margin">4% above market</p>
          <p class="value">COL$ 3.695,59</p>
        </div>
      </div>

      <div class="sumary">
        <p class="label">Transaction sumary</p>
        <div class="wrap">
          <div class="item">
            <p class="info">Trading Fee</p>
            <p>COL$ 3.695,59</p>
          </div>
          <div class="item">
            <p class="info">You will get</p>
            <p class="price-get">100.00 UST</p>
          </div>
          <div class="item">
            <p class="info">You will pay</p>
            <p class="price-pay">COP$ 348.892,53</p>
          </div>
        </div>
      </div>
    </div>

    <div class="wrap-btns">
      <button class="secondary">cancel</button>
      <button class="primary">open transaction</button>
    </div>
  </div>
  <!-- Expanded Offer Mobile -->
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import { formatAddress, formatAmount } from "@/shared";
import ExpandedOffer from "@/components/ExpandedOffer.vue";

export default defineComponent({
  name: "Offers",
  components: {
    ExpandedOffer,
  },
  data() {
    return {
      offerTypeLabels: { buy: "Sell", sell: "Buy" },
      expandedOffer: null,
    };
  },
  methods: {
    ...mapActions(["fetchOffers", "fetchUsdRates", "openTrade"]),
    formatAmount,
    formatAddress,
    expandOfferItem: function(offer) {
      console.log("offer", offer);
      this.expandedOffer = offer
      //this.$data.expandedOffer = offer;
      /*
      this.$nextTick(() => {
        this.$refs.expandedOffer.focus()
      });
       */
    },
  },
  computed: {
    ...mapGetters(["offers", "getUsdRate"]),
    collapsedOffers: function() {
      return this.offers.filter((offer) => offer.id != this.expandedOffer);
    },
    expandedOffers: function() {
      return this.offers.filter((offer) => offer.id == this.expandedOffer);
    },
  },
  created() {
    this.fetchUsdRates();
  },
});
</script>

<style lang="scss" scoped>
@import "../style/tokens.scss";

/* ----------- BUY SELL ROW */

.separator {
  margin: 0 auto 42px;
  display: flex;
  height: 1px;
  background-color: $border;
}

.offers-filter {
  display: flex;

  @media only screen and (max-width: 550px) {
    display: block;
  }
}

.buy-sell {
  display: inline-flex;
  justify-content: space-evenly;
  width: 100%;
  max-width: 200px;
  margin-top: 26px;
  background-color: $surface;
  border: 1px solid $border;
  border-radius: 8px;
  overflow: hidden;

  .separator {
    width: 1px;
    height: 40px;
    background-color: $border;
    margin: 0px;
    z-index: 999;
  }

  button {
    width: 100%;
    height: 40px;
    background-color: $surface;
    font-size: 14px;
    font-weight: 600;
    color: $primary;
    border-radius: 0px;

    &:focus {
      background-color: $gray300;
      color: $primary;
    }
  }

  @media only screen and (max-width: 550px) {
    margin-top: 0;
    max-width: none;
    margin-bottom: 16px;

    button {
      height: 48px;
    }
    .separator {
      height: 48px;
    }
  }
}

.filter {
  display: inline-flex;
  flex-direction: column;
  width: 100%;
  max-width: 200px;
  margin-left: 24px;

  label {
    font-size: 12px;
    color: $gray600;
    margin-bottom: 8px;
  }

  select {
    width: 100%;
    max-width: 200px;
    background-color: $surface;
    border-radius: 8px;
    border: 1px solid $border;
    font-family: "Poppins", sans-serif;
    font-size: 14px;
    font-weight: 600;
    color: $base-text;
    padding: 10px 16px;
    appearance: none;
  }

  @media only screen and (max-width: 550px) {
    margin-left: 0;
    max-width: none;

    select {
      max-width: none;
      height: 48px;
    }
  }
}

/* ----------- OFFER LIST */

.offers-list {
  margin-top: 40px;

  h3 {
    color: $base-text;
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 32px;
  }

  li {
    list-style: none;
    padding: 24px 32px;
    margin-bottom: 24px;
    border: 1px solid $border;
    background-color: $surface;
    border-radius: 8px;
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

    .info {
      grid-column: 2/5;
      grid-row: 1;
      display: flex;
      align-items: center;
      gap: 24px;

      .currency {
        text-align: center;
        font-size: 12px;
        font-weight: 600;
        color: $gray600;
        text-transform: uppercase;

        img {
          width: 24px;
        }
      }

      .divider {
        height: 40px;
        width: 1px;
        background-color: $border;
      }

      .note {
        font-size: 14px;
        color: $gray600;
        max-width: 200px;
      }

      .min-max {
        font-size: 14px;
        color: $gray600;
      }

      @media only screen and (max-width: 550px) {
        grid-column: 1/7;
        grid-row: 2;

        .note {
          grid-row: 3;
        }
        .divider {
          display: none;
        }
        .min-max {
          padding: 8px 16px;
          margin-bottom: 4px;
          border-radius: 8px;
          background-color: $gray150;
        }
      }
    }
  }

  .collapsed {
    .price {
      grid-column: 1/7;
      grid-row: 1;
      justify-self: end;
      text-align: right;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 24px;

      .value {
        font-size: 20px;
        font-weight: 800;
        color: $base-text;
      }
      .margin {
        font-size: 14px;
        color: $gray600;
      }
      button {
        background-color: $gray300;
        color: $primary;
        border: none;
        font-family: inherit;
        font-weight: 700;
        font-size: 16px;
        text-transform: lowercase;
        padding: 8px 24px;
      }

      @media only screen and (max-width: 550px) {
        grid-column: 1/7;
        grid-row: 3;
        text-align: left;
        justify-content: space-between;
        gap: none;
        width: 100%;
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
        width: 100%;
        font-family: "Poppins", sans-serif;
        font-size: 16px;
        font-weight: 800;
        line-height: 24px;
        color: $base-text;
        padding: 10px 16px;
        border: 1px solid $border;
        background-color: $gray300;
        border-radius: 8px;
        margin-top: 8px;
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

      .sumary {
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
}

/* -------------- Expanded Mobile */

.expanded-mobile {
  position: absolute;
  width: 100%;
  height: 100vh;
  display: grid;
  grid-template-columns: 1fr;
  background-color: $white;

  .owner {
    grid-column: 1/1;
    grid-row: 1;
    padding: 16px 24px 0;

    .wallet {
      font-size: 18px;
      font-weight: 600;
      color: $base-text;
    }
    .n-trades {
      font-size: 14px;
      color: $gray600;
    }
  }

  .payment-info {
    grid-column: 1/1;
    grid-row: 2;
    padding: 0px 24px;

    .note {
      font-size: 14px;
      color: $gray600;
    }
  }

  form {
    grid-column: 1/1;
    grid-row: 3;
    margin-top: 16px;
    padding: 0px 24px;

    .input:first-child {
      margin-bottom: 24px;
    }

    label {
      font-size: 14px;
      color: $gray600;
      display: block;
    }

    input {
      width: 100%;
      font-family: "Poppins", sans-serif;
      font-size: 16px;
      font-weight: 800;
      line-height: 24px;
      color: $base-text;
      padding: 10px 16px;
      border: 1px solid $border;
      border-radius: 8px;
      margin-top: 8px;
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
    grid-column: 1/1;
    grid-row: 4;
    margin-top: 16px;
    background-color: $background;
    border-top: 1px solid $border;
    border-bottom: 1px solid $border;
    padding: 16px 24px;

    .price {
      margin-bottom: 24px;

      .wrap-price {
        display: flex;
        justify-content: space-between;
      }

      .label {
        display: inline-block;
        font-size: 14px;
        color: $gray600;
      }

      .ticker {
        display: inline-block;
        font-size: 12px;
        color: $primary;
      }

      .wrap {
        width: 100%;
        display: inline-flex;
        justify-content: space-between;
        border: 1px solid $border;
        background-color: $white;
        border-radius: 8px;
        padding: 10px 24px;
        margin-top: 8px;
        align-items: center;
        gap: 16px;

        .margin {
          font-size: 14px;
          max-width: 100px;
          color: $gray600;
        }
        .value {
          font-size: 16px;
          font-weight: 600;
          color: $base-text;
        }
      }
    }

    .sumary {
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
        border: 1px solid $border;
        background-color: $white;
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
  }
}

.wrap-btns {
  grid-template-columns: 1/1;
  grid-row: 5;
  display: flex;
  justify-content: space-around;
  padding: 24px 0px;

  .primary {
    background-color: #ef6100;
    color: $white;
    border: none;
    font-family: inherit;
    font-weight: 700;
    font-size: 16px;
    text-transform: lowercase;
    padding: 8px 24px;
  }

  .secondary {
    color: $primary;
    border: none;
    font-family: inherit;
    font-weight: 700;
    font-size: 16px;
    text-transform: lowercase;
    padding: 8px 24px;
  }
}
</style>
