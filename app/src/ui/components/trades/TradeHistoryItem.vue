<script setup lang="ts">
import { formatAddress, formatAmount, formatDate, formatFiatAmount, formatTradeState } from '~/shared'
import type { TradeInfo } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { microDenomToDenom } from '~/utils/denom'

const props = defineProps<{ tradeInfo: TradeInfo }>()
const client = useClientStore()
const currentDate = computed(() => formatDate(new Date(props.tradeInfo.trade.created_at * 1000)))
const fiatCurrency = computed(() => props.tradeInfo.offer.offer.fiat_currency)
const fiatAmountStr = computed(() => {
  const fiatAmount = formatFiatAmount(
    (parseInt(props.tradeInfo.trade.amount) / 1000000) * (props.tradeInfo.trade.denom_fiat_price / 100)
  )
  return `${fiatCurrency.value} ${fiatAmount}`
})

const tradeType = computed(() => {
  return props.tradeInfo.offer.offer.offer_type === 'buy' ? 'Buy' : 'Sell'
})

const counterparty = computed(() => {
  const trade = props.tradeInfo.trade
  return client.userWallet.address === trade.seller ? trade.buyer : trade.seller
})
</script>

<template>
  <div class="wrap-table-item">
    <div class="col-1 trade-id">
      <router-link :to="`/trade/${props.tradeInfo.trade.id}`">
        <p>{{ props.tradeInfo.trade.id }}</p>
      </router-link>
    </div>
    <div class="col-2">
      <p>{{ currentDate }}</p>
    </div>
    <div class="col-3">
      <p>{{ tradeType }}</p>
    </div>
    <div class="col-4">
      <p>{{ formatAmount(tradeInfo.trade.amount) }} {{ microDenomToDenom(tradeInfo.trade.denom.native) }}</p>
    </div>
    <div class="col-5">
      <p>{{ fiatAmountStr }}</p>
    </div>
    <div class="col-6 trader">
      <p>{{ formatAddress(counterparty) }}</p>
    </div>
    <div class="col-7">
      <p>{{ formatTradeState(tradeInfo.trade.state) }}</p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.wrap-table-item {
  display: flex;
  flex-direction: row;
  padding: 16px;

  p {
    font-size: 14px;
    font-weight: $regular;
  }

  .trade-id a {
    color: $primary;
  }

  .trader {
    color: $primary;
  }
  @media only screen and (max-width: $mobile) {
    min-width: 1000px;
    padding: 8px 0 16px 0;
  }
}
</style>
