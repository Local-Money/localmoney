<script setup lang="ts">
import { formatAddress, formatAmount, formatDate, formatFiatAmount, formatTradeState } from '~/shared'
import type { TradeInfo } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { microDenomToDisplay } from '~/utils/denom'

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
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
          <path
            d="M15 7H18C18.6566 7 19.3068 7.12933 19.9134 7.3806C20.52 7.63188 21.0712 8.00017 21.5355 8.46447C21.9998 8.92876 22.3681 9.47996 22.6194 10.0866C22.8707 10.6932 23 11.3434 23 12C23 12.6566 22.8707 13.3068 22.6194 13.9134C22.3681 14.52 21.9998 15.0712 21.5355 15.5355C21.0712 15.9998 20.52 16.3681 19.9134 16.6194C19.3068 16.8707 18.6566 17 18 17H15M9 17H6C5.34339 17 4.69321 16.8707 4.08658 16.6194C3.47995 16.3681 2.92876 15.9998 2.46447 15.5355C1.52678 14.5979 1 13.3261 1 12C1 10.6739 1.52678 9.40215 2.46447 8.46447C3.40215 7.52678 4.67392 7 6 7H9"
            stroke="inherit"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path d="M8 12H16" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
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
      <p>
        {{ formatAmount(tradeInfo.trade.amount, true, 6) }}
        {{ microDenomToDisplay(tradeInfo.trade.denom.native, client.chainClient) }}
      </p>
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
  color: $gray900;

  p {
    font-size: 14px;
    font-weight: $regular;
  }

  .trade-id {
    a {
      display: flex;
      gap: 8px;
      color: $gray900;

      &:hover {
        color: $primary;
        stroke: $primary;
      }
      svg {
        margin-top: 2px;
        width: 16px;
        height: 16px;
        stroke: $gray900;
      }
    }
  }

  @media only screen and (max-width: $mobile) {
    min-width: 1000px;
    padding: 8px 0 16px 0;
  }
}
</style>
