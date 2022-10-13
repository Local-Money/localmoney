<script setup lang="ts">
import { formatAddress, formatAmount } from '~/shared'
import { useClientStore } from '~/stores/client'
import type { TradeInfo } from '~/types/components.interface'
import { microDenomToDenom } from '~/utils/denom'

const props = defineProps<{ tradeInfo: TradeInfo }>()
const client = useClientStore()
let refreshTradeDetailInterval: NodeJS.Timer | undefined
const secondsUntilTradeDetailRefresh = ref(0)
const walletAddress = computed(() => client.userWallet.address)
const stepLabels = {
  buy: {
    buyer: [
      'Review trade request',
      'Waiting for funds',
      'Please make the payment',
      'Waiting for funds release',
      'Trade finished',
      'In dispute',
    ],
    seller: [
      'Waiting for buyer',
      'Please fund the trade',
      'Waiting for payment',
      'Please release the funds',
      'Trade finished',
      'In dispute',
    ],
  },
  sell: {
    buyer: [
      'Waiting for funds',
      'Please make the payment',
      'Waiting for funds release',
      'Trade finished',
      'In dispute',
    ],
    seller: [
      'Please fund the trade',
      'Waiting for payment',
      'Please release the funds',
      'Trade finished',
      'In dispute',
    ],
  },
}

const step = computed(() => {
  const trade = props.tradeInfo.trade
  if (props.tradeInfo.offer.offer.offer_type === 'buy') {
    switch (trade.state) {
      case 'request_created':
        return 1
      case 'request_accepted':
        return 2
      case 'escrow_funded':
        return 3
      case 'fiat_deposited':
        return 4
      case 'escrow_released':
        return 5
      case 'escrow_disputed':
        return 6
      default:
        return 0
    }
  } else {
    switch (trade.state) {
      case 'request_created':
        return 1
      case 'escrow_funded':
        return 2
      case 'fiat_deposited':
        return 3
      case 'escrow_released':
        return 4
      case 'escrow_disputed':
        return 5
      default:
        return 0
    }
  }
})

const counterparty = computed(() => {
  const trade = props.tradeInfo.trade
  return walletAddress.value === trade.seller ? trade.buyer : trade.seller
})

const isBuying = computed(() => {
  return props.tradeInfo.trade.seller !== walletAddress.value
})

const buyOrSell = computed(() => {
  return isBuying.value ? 'Buy' : 'Sell'
})

const fromTo = computed(() => {
  return isBuying.value ? 'from' : 'to'
})

const stepLabel = computed(() => {
  const labelIdx = step.value - 1
  const type = props.tradeInfo.offer.offer.offer_type
  if (isBuying.value) {
    return stepLabels[type].buyer[labelIdx]
  } else {
    return stepLabels[type].seller[labelIdx]
  }
})

function startTradeDetailRefresh() {
  let seconds = 60
  const countdownInterval = 1000
  refreshTradeDetailInterval = setInterval(async () => {
    secondsUntilTradeDetailRefresh.value = --seconds
    if (seconds === 0) {
      await client.fetchTradeDetail(props.tradeInfo.trade.id)
      seconds = 60
    }
  }, countdownInterval)
}

onMounted(() => {
  nextTick(() => {
    startTradeDetailRefresh()
  })
})

onUnmounted(() => {
  clearInterval(refreshTradeDetailInterval)
})
</script>

<template>
  <div class="card offer collapsed" v-bind="(trade = tradeInfo.trade)">
    <div class="trade-type">
      <p class="type">{{ buyOrSell }}ing {{ microDenomToDenom(trade.denom.native) }}</p>
      <p class="wallet-addr">{{ fromTo }} {{ formatAddress(counterparty) }}</p>
    </div>

    <div class="info">
      <div class="wrap">
        <p class="label">Status</p>
        <p class="content">
          {{ stepLabel }}
        </p>
      </div>

      <div class="divider" />

      <div class="wrap">
        <p class="label">Amount</p>
        <p class="content">
          {{ formatAmount(trade.amount) }}
          {{ microDenomToDenom(trade.denom.native) }}
        </p>
      </div>

      <div class="divider" />

      <div class="wrap">
        <p class="label">Time remaining</p>
        <p class="content">?? min</p>
      </div>
    </div>

    <div class="price">
      <router-link :to="`/trade/${trade.id}`">
        <button class="primary bg-gray300">view trade</button>
      </router-link>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.collapsed {
  margin-bottom: 24px;

  .trade-type {
    display: flex;
    flex-direction: column;

    .type {
      font-size: 18px;
      font-weight: 600;
      color: $base-text;
    }

    .wallet-addr {
      font-size: 14px;
      color: $gray700;
      margin-top: 4px;
    }
  }
}
</style>
