<script setup lang="ts">
import { storeToRefs } from 'pinia'
import TradeOpenItem from '../components/trades/TradeOpenItem.vue'
import TradeHistoryItem from '../components/trades/TradeHistoryItem.vue'
import { useClientStore } from '~/stores/client'
import { TradeState } from '~/types/components.interface'
import { checkTradeNeedsRefund, checkValidOffer } from '~/utils/validations'
import { Page, trackPage } from '~/analytics/analytics'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
const tradeResult = computed(() => client.trades)
const trades = computed(() => {
  if (tradeResult.value.isSuccess()) {
    return tradeResult.value.data.filter((trade) => {
      return checkValidOffer(trade.offer.offer, client.chainClient)
    })
  } else {
    return []
  }
})

const openTrades = computed(() => {
  return trades.value.filter((tradeInfo) => {
    return (
      [
        TradeState.request_created,
        TradeState.request_accepted,
        TradeState.escrow_funded,
        TradeState.fiat_deposited,
        TradeState.escrow_disputed,
      ].includes(tradeInfo.trade.state) || checkTradeNeedsRefund(tradeInfo.trade, userWallet.value.address)
    )
  })
})

const closedTrades = computed(() => {
  return trades.value.filter((tradeInfo) => {
    return (
      [
        TradeState.request_canceled,
        TradeState.request_expired,
        TradeState.escrow_refunded,
        TradeState.escrow_released,
        TradeState.settled_for_maker,
        TradeState.settled_for_taker,
      ].includes(tradeInfo.trade.state) && !checkTradeNeedsRefund(tradeInfo.trade, userWallet.value.address)
    )
  })
})

const hasOpenTrades = computed(() => openTrades.value.length > 0)
const hasClosedTrades = computed(() => closedTrades.value.length > 0)
const paginationLastItem = ref<number>(0)

onMounted(() => {
  nextTick(async () => await client.fetchTrades())
  trackPage(Page.my_trades)
})

onUnmounted(async () => {})

watch(userWallet, () => {
  nextTick(async () => await client.fetchTrades())
})

async function loadMore() {
  const lastIndex = trades.value.length
  paginationLastItem.value = lastIndex > 0 ? trades.value[lastIndex - 1].trade.id : 0
  await client.fetchMoreTrades(paginationLastItem.value)
}
</script>

<template>
  <main class="page">
    <div class="wrap-title">
      <h3>Open Trades</h3>
    </div>
    <!-- Open Trades section -->
    <ListContentResult :result="tradeResult" emptyStateMsg="There are no trades here yet" @loadMore="loadMore()">
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
.wrap-title {
  display: flex;
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
