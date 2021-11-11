<template>
  <div class="main-wrap">
    <div class="buy-sell">
      <button
        v-on:click="offerType = 0"
        v-bind:class="{ focus: offerType == 0 }"
      >
        Buy
      </button>
      <div class="separator"></div>
      <button
        v-on:click="offerType = 1"
        v-bind:class="{ focus: offerType == 1 }"
      >
        Sell
      </button>
    </div>
    <div class="currency card">
      <div class="filter">
        <label for="crypto">Crypto</label>
        <select name="crypto" id="crypto">
          <option value="UST">UST</option>
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
    </div>
    <div class="min-max card">
      <div class="wrap">
        <label>Min amount:</label>
        <input type="text" v-model="minAmount" />
      </div>
      <div class="wrap">
        <label>Max amount:</label>
        <input type="text" v-model="maxAmount" />
      </div>
    </div>
    <div class="create">
      <button class="create-btn" @click="createOffer()">Create</button>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import { formatAddress, formatAmount } from "@/shared";
import { mapActions, mapGetters } from "vuex";

export default defineComponent({
  name: "CreateOffer",
  data() {
    return {
      minAmount: 10000000,
      maxAmount: 500000000,
      offerType: 1,
    };
  },
  methods: {
    ...mapActions(["initWallet", "newOffer"]),
    formatAmount,
    formatAddress,
    createOffer() {
      let offerType = this.offerType === 0 ? "buy" : "sell";
      const newOffer = {
        create: {
          offer: {
            offer_type: offerType,
            fiat_currency: "BRL",
            min_amount: parseInt(this.minAmount),
            max_amount: parseInt(this.maxAmount),
          },
        },
      };
      this.newOffer({ offer: newOffer });
    },
  },
  computed: mapGetters(["walletAddress"]),
  created() {
    this.initWallet();
  },
});
</script>

<style lang="scss" scoped>
@import "../../style/tokens.scss";
@import "../../style/elements.scss";

.main-wrap {
  display: inline-flex;
  flex-direction: column;
}

.buy-sell {
  display: flex;
  margin-top: 40px;
}

.min-max {
  margin-top: 24px;
  display: inline-flex;
  flex-basis: content;

  .wrap {
    display: flex;
    flex-direction: column;

    &:last-child {
      margin-left: 24px;
    }

    label {
      font-size: 14px;
      font-weight: 400;
      color: $gray900;
      margin-bottom: 8px;
    }
  }

  input {
    width: 100%;
    background-color: $background;
  }
}

.create {
  display: flex;

  .create-btn {
    padding: 16px 24px;
    margin-top: 24px;
    background-color: $surface;
    font-size: 16px;
    font-weight: 700;
    line-height: 0px;
    color: $primary;
    border-radius: 8px;
  }
}

.currency {
  display: flex;
  margin-top: 24px;
  .filter {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 200px;

    &:last-child {
      margin-left: 24px;
    }

    label {
      font-size: 14px;
      font-weight: 400;
      color: $gray900;
      margin-bottom: 8px;
    }

    select {
      width: 100%;
      max-width: 200px;
      background-color: $gray300;
      border-radius: 8px;
      border: 1px solid $border;
      font-family: "Poppins", sans-serif;
      font-size: 14px;
      font-weight: 600;
      color: $base-text;
      padding: 10px 16px;
      appearance: none;
      outline: none;
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
}
</style>
