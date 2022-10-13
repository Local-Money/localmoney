<script setup lang="ts">
import { storeToRefs } from 'pinia'
import TradeOpenItem from '../components/trades/TradeOpenItem.vue'
import TradeHistoryItem from '../components/trades/TradeHistoryItem.vue'
import { useClientStore } from '~/stores/client'
import { TradeState } from '~/types/components.interface'
import { checkValidOffer } from '~/utils/validations'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
const tradeResult = computed(() => client.trades)
const trades = computed(() => {
  if (tradeResult.value.isSuccess()) {
    return tradeResult.value.data.filter((trade) => checkValidOffer(trade.offerResponse.offer))
  } else {
    return []
  }
})

const openTrades = computed(() => {
  return trades.value.filter(
    (tradeInfo) =>
      !tradeInfo.expired &&
      [
        TradeState.request_created,
        TradeState.request_accepted,
        TradeState.escrow_funded,
        TradeState.fiat_deposited,
        TradeState.escrow_disputed,
      ].includes(tradeInfo.trade.state)
  )
})

const closedTrades = computed(() => {
  return trades.value.filter(
    (tradeInfo) =>
      tradeInfo.expired ||
      [
        TradeState.request_canceled,
        TradeState.request_expired,
        TradeState.escrow_refunded,
        TradeState.escrow_released,
        TradeState.settled_for_maker,
        TradeState.settled_for_taker,
      ].includes(tradeInfo.trade.state)
  )
})

const hasOpenTrades = computed(() => openTrades.value.length > 0)
const hasClosedTrades = computed(() => closedTrades.value.length > 0)

onMounted(() => {
  nextTick(async () => await client.fetchMyTrades())
})

onUnmounted(async () => {})

watch(userWallet, () => {
  nextTick(async () => await client.fetchMyTrades())
})
</script>

<template>
  <main class="page">
    <h3>Open Trades</h3>
    <!-- Open Trades section -->
    <ListContentResult :result="tradeResult" emptyStateMsg="There are no trades here yet">
      <section v-if="hasOpenTrades">
        <TradeOpenItem v-for="tradeInfo in openTrades" :key="tradeInfo.trade.addr" :tradeInfo="tradeInfo" />
      </section>
      <section v-else class="card">
        <p>Nothing here yet</p>
      </section>
      <!-- End Open Trades section -->
      <!-- Trades History section -->
      <h3 v-if="hasClosedTrades">Trade History</h3>
      <section v-if="hasClosedTrades" class="trade-history-table card">
        <div class="table-header">
          <div class="col-1">
            <p>ID</p>
          </div>
          <div class="col-2">
            <p>Date</p>
          </div>
          <div class="col-3">
            <p>Type</p>
          </div>
          <div class="col-4">
            <p>Crypto</p>
          </div>
          <div class="col-5">
            <p>Amount</p>
          </div>
          <div class="col-6">
            <p>Trader</p>
          </div>
          <div class="col-7">
            <p>Status</p>
          </div>
        </div>
        <TradeHistoryItem v-for="tradeInfo in closedTrades" :key="tradeInfo.trade.addr" :tradeInfo="tradeInfo" />
      </section>
    </ListContentResult>
    <!-- End Trades History section -->
  </main>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

main {
  margin-bottom: 48px;
}

h3 {
  margin: 32px 0;
  font-size: 18px;
  font-weight: $semi-bold;
}

.trade-history-table {
  overflow: auto;
  .table-header {
    display: flex;
    flex-direction: row;
    border-bottom: 1px solid $border;
    padding: 16px;
    margin-bottom: 16px;

    p {
      font-size: 14px;
      font-weight: $semi-bold;
      color: $gray700;
    }

    @media only screen and (max-width: $mobile) {
      min-width: 1000px;
      padding: 0 0 16px 0;
    }
  }
}

.col-1,
:deep(.col-1) {
  width: 10%;
}

.col-2,
:deep(.col-2) {
  width: 20%;
}

.col-3,
:deep(.col-3) {
  width: 7.5%;
}

.col-4,
:deep(.col-4) {
  width: 17%;
}

.col-5,
:deep(.col-5) {
  width: 17%;
}

.col-6,
:deep(.col-6) {
  width: 20%;
}

.col-7,
:deep(.col-7) {
  width: 10%;
}
</style>
