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
                <p class="list-item-label" v-if="isBuying">You will get</p>
                <p class="list-item-label" v-else>You will send</p>
                <p class="value">{{ formatAmount(trade.ust_amount) }}UST</p>
              </div>
              <div class="list-item">
                <p class="list-item-label" v-if="isBuying">You will send</p>
                <p class="list-item-label" v-else>You will get</p>
                <p class="value fiat">{{ fiatAmountStr }}</p>
              </div>
            </div>
          </div>
        </section>
        <section class="actions card">

          <!-- Step 1 -->
          <!-- Fund Escrow -->
          <div class="wrap" v-if="tradeCanBeFunded(tradeInfo, this.walletAddress)">
            <div class="icon">
              <RightArrow/>
            </div>
            <p>To begin the transaction you have to fund the escrow</p>
            <button @click="this.fundEscrow(trade.addr)">
              fund escrow
            </button>
          </div>
          <div class="wrap" v-else-if="trade.state === 'created'">
            <div class="icon">
              <RightArrow/>
            </div>
            <p>Wait until the trader funds the escrow</p>
          </div>

          <!-- Step 2 -->
          <!-- Mark as Paid -->
          <div class="wrap" v-if="trade.state === 'escrow_funded' && this.isBuying && !trade.paid">
            <div class="icon">
              <RightArrow/>
            </div>
            <p v-if="trade.paid">You notified the trade about the off-chain payment</p>
            <p v-else>Notify the trader that you made the off-chain payment</p>
            <button @click="this.markAsPaid(trade)" v-if="!trade.paid">
              mark as paid
            </button>
          </div>

          <!-- Wait for Off-chain Payment -->
          <div class="wrap" v-if="trade.state === 'escrow_funded' && !this.isBuying && !trade.paid">
            <div class="icon">
              <RightArrow/>
            </div>
            <p @click="log(trade)">Wait until the trader makes the off-chain payment</p>
          </div>

          <!-- Step 3 -->
          <!-- Release Escrow -->
          <div class="wrap" v-if="tradeCanBeReleased(tradeInfo, this.walletAddress) && trade.paid">
            <div class="icon">
              <RightArrow/>
            </div>
            <p>Check if you received the off-chain payment before releasing the escrow</p>
            <button @click="this.releaseEscrow(trade.addr)">
              release escrow
            </button>
          </div>

          <!-- Wait for Escrow Release -->
          <div class="wrap" v-else-if="trade.paid && trade.state === 'escrow_funded'">
            <div class="icon">
              <RightArrow/>
            </div>
            <p>Wait until trader releases the escrow, you'll receive it on your wallet</p>
          </div>

          <!-- Trade Closed -->
          <div class="wrap" v-if="trade.state === 'closed'">
            <div class="icon">
              <IconDone/>
            </div>
            <p>Trade finished</p>
          </div>

        </section>
      </div>
    </section>
  </main>
</template>

<script>
import IconDone from "@/components/commons/IconDone";
import RightArrow from "@/components/commons/RightArrow";
import {defineComponent} from "vue";
import {mapGetters, mapActions} from "vuex";
import {tradesCollection, updateTrade} from "../store/firebase";
import {onSnapshot} from "firebase/firestore"
import {
  formatAmount,
  formatAddress,
  tradeCanBeFunded,
  tradeCanBeReleased,
  tradeCanBeRefunded,
} from "../shared";

export default defineComponent({
  name: "TradeDetail",
  components: {
    IconDone,
    RightArrow
  },
  methods: {
    ...mapActions(["fundEscrow", "releaseEscrow", "fetchTradeInfo", "setTradeAsPaid"]),
    formatAmount,
    formatAddress,
    tradeCanBeFunded,
    tradeCanBeReleased,
    tradeCanBeRefunded,
    markAsPaid: function () {
      const trade = this.tradeInfo.trade
      trade.paid = true
      this.setTradeAsPaid({tradeAddr: trade.addr, paid: true})
      updateTrade(this.tradeInfo.trade)
    },
  },
  computed: {
    ...mapGetters(["getTradeInfo", "walletAddress", "getUsdRate"]),
    tradeInfo: function () {
      return this.getTradeInfo(this.$route.params.addr)
    },
    isBuying: function () {
      return this.tradeInfo.trade.recipient === this.walletAddress
    },
    buyOrSell: function () {
      return this.isBuying ? "Buy" : "Sell"
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
      console.log('fiat', this.fiatCurrency);
      const fiatAmount = formatAmount(this.getUsdRate(this.fiatCurrency), false);
      return `${this.fiatCurrency} ${fiatAmount}`;
    }
  },
  mounted: async function () {
    const trade = this.tradeInfo.trade
    const tradeAddr = trade.addr
    this.unsubscribe = onSnapshot(tradesCollection.doc(tradeAddr), (doc) => {
      let data = doc.data()
      if (data && data.state === "closed" && trade.state !== "closed") {
        this.$nextTick(() => {
          this.fetchTradeInfo({addr: tradeAddr, tradeData: data})
        })
      } else if (data && data.paid !== undefined && trade.paid !== data.paid) {
        this.setTradeAsPaid({tradeAddr, paid: data.paid})
      } else {
        this.$nextTick(() => {
          this.fetchTradeInfo({addr: tradeAddr, tradeData: data})
        })
      }
    })
  },
  unmounted: function () {
    if (this.unsubscribe) {
      this.unsubscribe()
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
