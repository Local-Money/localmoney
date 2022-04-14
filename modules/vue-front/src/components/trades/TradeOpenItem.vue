<template>
  <div class="open-trade-item card" v-bind="trade = this.tradeInfo.trade">
    <p class="trade-type">{{ buyOrSell }}ing UST {{ fromTo }} {{ formatAddress(counterparty) }}</p>
    <p class="trade-value">{{ formatAmount(trade.ust_amount) }} UST</p>
    <span class="separator"></span>
    <div class="wrap-status">
      <div class="column-1">
        <p class="step">Step {{ step }} of 4</p>
        <p class="status">{{ stepLabel }}</p>
      </div>
      <div class="column-2">
        <p class="time-label">Time remaining</p>
        <p class="time">30 min</p>
      </div>
    </div>
    <router-link :to="`/trade/${trade.addr}`">
      <button>view ></button>
    </router-link>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {mapActions, mapGetters} from "vuex";
import {formatAddress, formatAmount} from '@/shared'
import {onSnapshot} from "firebase/firestore";
import {tradesCollection} from "@/store/firebase";

export default defineComponent({
  name: 'TradeOpenItem',
  props: ['tradeAddr'],
  data: function () {
    return {
      stepLabels: {
        buyer: ['Wait for the escrow', 'Notify about the off-chain payment',
          'Wait for escrow release', 'Trade finished'],
        seller: ['Fund the escrow', 'Wait the off-chain payment',
          'Confirm the receipt by releasing the escrow', 'Trade finished']
      }
    }
  },
  computed: {
    ...mapGetters(["getUsdRate", "walletAddress", "getTradeInfo"]),
    tradeInfo: function () {
      return this.getTradeInfo(this.$props.tradeAddr)
    },
    counterparty: function () {
      const trade = this.tradeInfo.trade
      return this.walletAddress === trade.seller ? trade.buyer: trade.seller;
    },
    isBuying: function () {
      return this.tradeInfo.trade.seller !== this.walletAddress
    },
    buyOrSell: function () {
      return this.isBuying ? "Buy" : "Sell"
    },
    fromTo: function () {
      return this.isBuying ? "from" : "to"
    },
    step: function () {
      const trade = this.tradeInfo.trade
      if (trade.state === "created") {
        return 1
      } else if ((trade.state === "escrow_funded")) {
        if (trade.paid) {
          return 3
        } else {
          return 2
        }
      } else {
        return 4
      }
    },
    stepLabel: function () {
      const labelIdx = this.step - 1
      return this.isBuying ? this.stepLabels.buyer[labelIdx] :
          this.stepLabels.seller[labelIdx]
    }
  },
  methods: {
    ...mapActions(["setTradeAsPaid", "fetchTradeInfo"]),
    formatAmount,
    formatAddress,
  },
  mounted: async function () {
    const tradeAddr = this.$props.tradeAddr
    const trade = this.tradeInfo.trade
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
})
</script>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.open-trade-item {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.trade-type,
.trade-value {
  flex-grow: 3;
}

.trade-type {
  font-size: 18px;
  font-weight: $semi-bold;
}

.trade-value {
  font-size: 14px;
  text-align: right;
  margin-right: 48px;
}

.wrap-status {
  flex-grow: 10;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-right: 32px;

  .step,
  .time-label {
    font-size: 12px;
    color: $gray600;
  }

  .status,
  .time {
    font-size: 14px;
    color: $gray700;
  }

  .column-2 {
    text-align: right;
  }
}

.separator {
  flex-grow: 2;
  border-left: 1px solid $border;
  display: block;
  position: relative;
  width: 1px;
  height: 40px;
}

button {
  height: 40px;
  background-color: $gray300;
  font-size: 16px;
  font-weight: 600;
  color: $primary;
  border-radius: 8px;
  padding: 8px 24px;

  &:focus {
    background-color: $gray300;
    color: $primary;
  }
}
</style>
