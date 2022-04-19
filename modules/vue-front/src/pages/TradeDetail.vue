<template>
  <main v-if="tradeInfo" v-bind="(trade = tradeInfo.trade)">
    <h3>{{ buyOrSell }}ing UST from {{ formatAddress(counterparty) }}</h3>
    <section class="stepper card">
      <!-- Step 1 -->
      <div class="step-item">
        <IconDone v-if="trade.state === 'escrow_funded' || trade.state === 'closed'"/>
        <div class="icon" v-else>
          <div class="counter">
            <p>1</p>
          </div>
        </div>
        <p>seller puts UST in escrow</p>
      </div>

      <!-- Step 2 -->
      <div class="step-item">
        <IconDone v-if="trade.paid || trade.state === 'closed'"/>
        <div class="icon" v-else>
          <div class="counter">
            <p>2</p>
          </div>
        </div>
        <p>buyer pays seller directly</p>
      </div>

      <!-- Step 3 -->
      <div class="step-item">
        <IconDone v-if="trade.state === 'closed'"/>
        <div class="icon" v-else>
          <div class="counter">
            <p>3</p>
          </div>
        </div>
        <p>escrow released to buyer</p>
      </div>

      <div class="step-status">
        <div class="separator"></div>
        <div class="wrap">
          <p>time remaining</p>
          <p class="step-time-left">30 min</p>
        </div>
        <div class="icon">
          <svg
              class="icon-24"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
          >
            <path
                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M12 6V12L16 14"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
          </svg>
        </div>
      </div>
    </section>
    <section class="wrap">
      <section class="chat card">Chat will be here</section>
      <div class="inner-wrap">
        <section class="trade-summary card">
          <div class="trader-info">
            <p><small>You're trading with</small></p>
            <p class="trader">{{ formatAddress(counterparty) }}</p>
            <p class="rating">0 trades</p>
          </div>
          <div class="trade-info">
            <p class="label">UST Price</p>
            <div class="current-price">
              <p class="mkt-rate">0% above market</p>
              <p class="price">{{ priceStr }}</p>
            </div>
            <p class="label">Transaction summary</p>
            <div class="transaction">
              <div class="list-item">
                <p class="list-item-label" v-if="isBuyer">You will get</p>
                <p class="list-item-label" v-else>You will send</p>
                <p class="value">{{ formatAmount(trade.ust_amount) }}UST</p>
              </div>
              <div class="list-item">
                <p class="list-item-label" v-if="isBuyer">You will send</p>
                <p class="list-item-label" v-else>You will get</p>
                <p class="value fiat">{{ fiatAmountStr }}</p>
              </div>
            </div>
          </div>
        </section>
        <section class="actions card">
          <!-- # If the user is buying UST (Buyer) -->
          <div v-if="isBuyer">
            <!-- #1 step (Optional) -->
            <!-- # A Seller requested a trade with the Buyer and it should be accepted first. -->
            <TradeAction
                v-if="this.tradeInfo.offer.offer_type === 'buy' && trade.state === 'request_created'"
                :message="'Accept the trade request to start.'"
                :button-label="'accept trade request'"
                @actionClick="this.acceptTradeRequest(trade.addr)"
            />
            <!-- #2 step or #1 step-->
            <!-- if #2 step: The Buyer accepted the request and needs to wait for the Seller to deposit UST on escrow-->
            <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the UST on escrow-->
            <TradeAction
                v-if="(this.tradeInfo.offer.offer_type === 'sell' && trade.state === 'request_created') || trade.state === 'request_accepted'"
                :message="'Waiting for the escrow to be funded'"
            />
            <!-- #3 step or #2 step-->
            <!-- The UST is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
            <TradeAction
                v-if="trade.state === 'escrow_funded'"
                :message="'Notify the trader that you made the off-chain payment'"
                :button-label="'mark as paid'"
                @actionClick="this.setFiatDeposited(trade.addr)"
            />
            <!-- #4 step or #3 step-->
            <!-- After the off-chain payment, the Buyer needs to wait for the Seller to release the funds on escrow -->
            <TradeAction
                v-if="trade.state === 'fiat_deposited'"
                :message="'Waiting for funds to be released.'"
            />
            <!-- #5 step or #4 step-->
            <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
            <TradeAction
                v-if="trade.state === 'escrow_released'"
                :message="'Trade finished successfully.'"
            />
          </div>

          <!-- # If the user is selling UST (Seller) -->
          <div v-if="isSeller">
            <!-- #1 step (Optional) -->
            <!-- # The Seller opens the trade with the Buyer and it should be accepted first. So the Seller needs to wait. -->
            <TradeAction
                v-if="this.tradeInfo.offer.offer_type === 'buy' && trade.state === 'request_created'"
                :message="'Wating for the Buyer to accept the trade request'"
            />
            <!-- #2 step or #1 step-->
            <!-- if #2 step: The Seller needs to deposit UST on escrow to enable the Buyer to transfer the Fiat-->
            <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the UST on escrow-->
            <TradeAction
                v-if="(this.tradeInfo.offer.offer_type === 'sell' && trade.state === 'request_created') || (trade.state === 'request_accepted')"
                :message="'To begin the transaction you have to fund the escrow'"
                :button-label="'fund escrow'"
                @actionClick="this.fundEscrow(trade.addr)"
            />
            <!-- #3 step or #2 step-->
            <!-- The UST is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
            <TradeAction
                v-if="trade.state === 'escrow_funded'"
                :message="'Waiting for fiat payment'"
            />
            <!-- #4 step or #3 step-->
            <!-- After the off-chain payment, the Seller needs to check the off-chain payment and release the UST on the escrow to the Buyer -->
            <TradeAction
                v-if="trade.state === 'fiat_deposited'"
                :message="'Check if you received the off-chain payment before releasing the escrow'"
                :button-label="'release escrow'"
                @actionClick="this.releaseEscrow(trade.addr)"
            />
            <!-- #5 step or #4 step-->
            <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
            <TradeAction
                v-if="trade.state === 'escrow_released'"
                :message="'Trade finished successfully.'"
            />
          </div>
        </section>
      </div>
    </section>
  </main>
</template>

<script>
import IconDone from "@/components/commons/IconDone";
import {defineComponent} from "vue";
import {mapActions, mapGetters} from "vuex";
import {tradesCollection} from "../store/firebase";
import {onSnapshot} from "firebase/firestore"
import {formatAddress, formatAmount} from "../shared";
import TradeAction from "@/components/tradeDetail/TradeAction";

export default defineComponent({
  name: "TradeDetail",
  components: {
    TradeAction,
    IconDone,
  },
  methods: {
    ...mapActions([
      "fundEscrow",
      "acceptTradeRequest",
      "releaseEscrow",
      "fetchTradeInfo",
      "fetchUsdRates",
      "setTradeAsPaid",
      "setFiatDeposited"
    ]),
    formatAmount,
    formatAddress
  },
  computed: {
    ...mapGetters([
      "getTradeInfo",
      "walletAddress",
      "getUsdRate"
    ]),
    tradeInfo: function () {
      return this.getTradeInfo(this.$route.params.addr)
    },
    isBuyer: function () {
      return this.tradeInfo.trade.buyer === this.walletAddress
    },
    isSeller: function () {
      return this.tradeInfo.trade.seller === this.walletAddress
    },
    buyOrSell: function () {
      return this.isBuyer ? "Buy" : "Sell"
    },
    counterparty: function () {
      const trade = this.tradeInfo.trade
      return this.walletAddress === trade.seller ? trade.buyer : trade.seller;
    },
    fiatCurrency: function () {
      return this.tradeInfo.offer.fiat_currency
    },
    fiatAmountStr: function () {
      const fiatAmount = formatAmount((this.tradeInfo.trade.ust_amount / 1000000)
          * this.getUsdRate(this.fiatCurrency), false)
      return `${this.fiatCurrency} ${fiatAmount}`
    },
    priceStr: function () {
      if (this.fiatCurrency) {
        const usdFiatRate = this.getUsdRate(this.fiatCurrency);
        const fiatAmount = formatAmount(usdFiatRate, false);
        return `${this.fiatCurrency} ${fiatAmount}`;
      } else {
        return '';
      }
    }
  },
  beforeMount: async function() {
    await this.fetchUsdRates();
    if (!this.tradeInfo) {
      await this.fetchTradeInfo({addr: this.$route.params.addr});
    }
  },
  fetchTrade: async function() {
    await this.fetchTradeInfo({addr: this.$route.params.addr});
  },
  mounted: async function () {
    if (this.tradeInfo && this.tradeInfo.trade) {
      const trade = this.tradeInfo.trade
      const tradeAddr = trade.addr
      this.unsubscribe = onSnapshot(tradesCollection.doc(tradeAddr), (doc) => {
        let data = doc.data()
        if (data && data.state !== trade.state) {
          this.$nextTick(() => {
            this.fetchTradeInfo({addr: tradeAddr, tradeData: data})
          })
        }
      })

      this.refreshInterval = setInterval(() => {
        this.fetchTrade();
      }, 5000);
    }
  },
  unmounted: function () {
    if (this.unsubscribe) {
      this.unsubscribe()
    }
    if (this.refreshInterval) {
      clearInterval(this.refreshInterval);
    }
  }
});
</script>

<style lang="scss" scoped>
@import "../style/pages.scss";

.stepper {
  display: flex;
  justify-content: space-between;
  padding: 24px 40px;
  margin-bottom: 24px;
}

.step-item,
.step-status {
  width: 20%;
  display: flex;
  align-items: center;
}

.step-item {
  .icon {
    margin-right: 24px;
  }

  .counter {
    width: 32px;
    height: 32px;
    border-radius: 100px;
    background-color: $border;
    text-align: center;
    padding-top: 6px;
    font-size: 14px;
    font-weight: $semi-bold;
  }

  p {
    font-size: 14px;
  }
}

.step-status {
  justify-content: flex-end;
  border-left: 1px solid $border;

  .wrap {
    text-align: right;
    margin-right: 24px;
    display: flex;
    flex-direction: column;

    p {
      font-size: 14px;
      color: $gray900;
    }

    .step-time-left {
      font-size: 18px;
      font-weight: $semi-bold;
      color: $primary;
    }
  }
}

.wrap {
  display: flex;
}

.chat {
  width: 30%;
  margin-right: 24px;
}

.inner-wrap {
  display: flex;
  flex-direction: column;
  width: 70%;
}

.trade-summary {
  display: flex;
  justify-content: space-evenly;

  .label {
    margin-bottom: 8px;
    font-size: 14px;
    color: $gray900;
  }

  .trader-info {
    width: 40%;

    .trader {
      font-size: 18px;
      font-weight: $semi-bold;
    }

    .rating {
      font-size: 14px;
      color: $gray900;
    }
  }

  .trade-info {
    width: 100%;

    .current-price,
    .transaction {
      background-color: $gray150;
      padding: 16px;
      border-radius: 8px;
    }

    .current-price {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 24px;

      .ticker {
        font-size: 12px;
        color: $gray900;
      }

      .mkt-rate {
        font-size: 14px;
        color: $gray900;
      }

      .price {
        font-size: 16px;
        font-weight: $semi-bold;
      }
    }

    .transaction {
      .list-item {
        display: flex;
        justify-content: space-between;
        align-items: center;

        &:first-child {
          margin-bottom: 8px;
        }

        p {
          font-size: 16px;
        }

        .value {
          font-weight: $semi-bold;
        }

        .fiat {
          color: $primary;
        }
      }
    }
  }
}

.actions {
  margin-top: 24px;

  .wrap {
    display: flex;
    align-items: center;

    .icon {
      stroke: $primary;
    }

    p {
      width: 50%;
      font-size: 16px;
      font-weight: 700;
      margin-left: 24px;
    }
  }
}

button {
  background-color: $gray300;
  color: $primary;
  margin-left: auto;
}
</style>
