<template>
  <div class="open-trade-item card">
    <p class="trade-type">Selling UST</p>
    <p class="trade-value">{{ formatAmount(trade.ust_amount) }} UST</p>
    <span class="separator"></span>
    <div class="wrap-status">
      <div class="column-1">
        <p class="step">Step 1 of 4</p>
        <p class="status">Waiting for Buyer</p>
      </div>
      <div class="column-2">
        <p class="time-label">Time remaining</p>
        <p class="time">30 min</p>
      </div>
    </div>
    <router-link :to="`/trade/terra13uflreu7tr4c0vykgwyy0ymgee9xs0r53xcphf`">
      <button>view ></button>
    </router-link>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {mapGetters} from "vuex";
import {formatAddress, formatAmount} from '@/shared'

export default defineComponent({
  name: 'TradeOpenItem',
  props: ['trade'],
  computed: {
    ...mapGetters(['getUsdRate']),
  },
  methods: {
    formatAmount,
    formatAddress
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
