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
          <option value="UST">mCOIN</option>
          <option value="UST">mTSLA</option>
          <option value="UST">LUNA</option>
          <option value="UST">LOTA</option>
          <option value="UST">LTT</option>
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
            <p class="note">
              Nubank, Itaú, C6, Mercado Pago, Inter, Caixa, Bradesco
            </p>
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
      this.$data.expandedOffer = offer;
      this.$nextTick(() => {
        //this.$refs.expandedOffer.focus()
      });
    },
  },
  computed: {
    ...mapGetters(["allOffers", "getUsdRate"]),
    collapsedOffers: function() {
      return this.allOffers.filter((offer) => offer.id != this.expandedOffer);
    },
    expandedOffers: function() {
      return this.allOffers.filter((offer) => offer.id == this.expandedOffer);
    },
  },
  created() {
    this.fetchOffers();
    this.fetchUsdRates();
  },
});
</script>

<style lang="scss">
@import "../style/offers.scss";
</style>
