<script setup lang="ts">
import { ref } from 'vue'
import { calculateFiatPriceByRate, convertOfferRateToMarginRate, formatAddress, formatAmount } from '~/shared'
import { useClientStore } from '~/stores/client'
import { usePriceStore } from '~/stores/price'
import { microDenomToDenom } from '~/utils/denom'

const client = useClientStore()
const priceStore = usePriceStore()
const tradeInfo = ref()

let refreshInterval: NodeJS.Timer

const route = useRoute()
const walletAddress = computed(() => client.userWallet.address)
const stepOneChecked = computed(() => {
  return (
    tradeInfo.value.trade.state === 'escrow_funded' ||
    tradeInfo.value.trade.state === 'fiat_deposited' ||
    tradeInfo.value.trade.state === 'escrow_released'
  )
})
const stepTwoChecked = computed(() => {
  return tradeInfo.value.trade.state === 'fiat_deposited' || tradeInfo.value.trade.state === 'escrow_released'
})
const stepThreeChecked = computed(() => tradeInfo.value.trade.state === 'escrow_released')
const isBuyer = computed(() => tradeInfo.value.trade.buyer === walletAddress.value)
const buyOrSell = computed(() => (isBuyer.value ? 'Buy' : 'Sell'))
const counterparty = computed(() => {
  const trade = tradeInfo.value.trade
  return walletAddress.value === trade.seller ? trade.buyer : trade.seller
})
const fiatCurrency = computed(() => tradeInfo.value.offer.fiat_currency)
const usdRate = computed(() => priceStore.getPrice(fiatCurrency.value))
const fiatPriceByRate = computed(() => calculateFiatPriceByRate(usdRate.value, tradeInfo.value.offer.rate))
const offerPrice = computed(() => `${fiatCurrency.value} ${formatAmount(fiatPriceByRate.value, false)}`)
const fiatAmountStr = computed(() => {
  const fiatAmount = formatAmount((parseInt(tradeInfo.value.trade.amount) / 1000000) * fiatPriceByRate.value, false)
  return `${fiatCurrency.value} ${fiatAmount}`
})
const marginRate = computed(() => convertOfferRateToMarginRate(tradeInfo.value.offer.rate))

function fetchTrade(id: string) {
  nextTick(async () => {
    tradeInfo.value = await client.fetchTradeDetail(id)
    refreshInterval = setInterval(async () => {
      tradeInfo.value = await client.fetchTradeDetail(id)
    }, 10 * 1000)
  })
}

onBeforeMount(() => {
  fetchTrade(route.params.id as string)
})

onMounted(() => {})

onUnmounted(() => {
  clearInterval(refreshInterval)
})
</script>

<template>
  <main v-if="tradeInfo" class="page" v-bind="(trade = tradeInfo.trade)">
    <h3>{{ buyOrSell }}ing {{ microDenomToDenom(trade.denom.native) }} from {{ formatAddress(counterparty) }}</h3>
    <section class="stepper card">
      <!-- Step 1 -->
      <div class="step-item">
        <IconDone v-if="stepOneChecked" />
        <div v-else class="icon">
          <div class="counter">
            <p>1</p>
          </div>
        </div>
        <p :class="stepOneChecked ? 'step-checked' : ''">waiting for funds</p>
      </div>

      <!-- Step 2 -->
      <div class="step-item">
        <IconDone v-if="stepTwoChecked" />
        <div v-else class="icon">
          <div class="counter">
            <p>2</p>
          </div>
        </div>
        <p :class="stepTwoChecked ? 'step-checked' : ''">waiting for payment</p>
      </div>

      <!-- Step 3 -->
      <div class="step-item">
        <IconDone v-if="stepThreeChecked" />
        <div v-else class="icon">
          <div class="counter">
            <p>3</p>
          </div>
        </div>
        <p :class="stepThreeChecked ? 'step-checked' : ''">waiting for funds release</p>
      </div>

      <div class="step-status">
        <div class="separator" />
        <div class="wrap">
          <p>time remaining</p>
          <p class="step-time-left">?? min</p>
        </div>
        <div class="icon">
          <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none">
            <path
              d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path d="M12 6V12L16 14" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
      </div>
    </section>
    <section class="main-wrap">
      <div class="chat card">Chat will be here</div>
      <div class="inner-wrap">
        <!-- Trade Summary -->
        <div class="trade-summary card">
          <div class="trader-info">
            <p><small>You're trading with</small></p>
            <p class="trader">
              {{ formatAddress(counterparty) }}
            </p>
            <p class="rating">0 trades</p>
          </div>
          <div class="trade-info">
            <p class="label">Price</p>
            <div class="current-price">
              <p class="mkt-rate">{{ marginRate.marginOffset }}% {{ marginRate.margin }} market</p>
              <p class="price">
                {{ offerPrice }}
              </p>
            </div>
            <p class="label">Transaction summary</p>
            <div class="transaction">
              <div class="list-item">
                <p v-if="isBuyer" class="list-item-label">You will get</p>
                <p v-else class="list-item-label">You will send</p>
                <p class="value">{{ formatAmount(trade.amount) }} {{ microDenomToDenom(trade.denom.native) }}</p>
              </div>
              <div class="list-item">
                <p v-if="isBuyer" class="list-item-label">You will send</p>
                <p v-else class="list-item-label">You will get</p>
                <p class="value fiat">
                  {{ fiatAmountStr }}
                </p>
              </div>
            </div>
          </div>
        </div>
        <!-- End Trade Summary -->

        <!-- TO-DO Consolidate this view into the above -->
        <!-- Trade Dispute Summary -->
        <!-- <div class="dispute-summary card">
          <div class="dispute-wrap">
            <div class="peer-wrap">
              <p class="peer">Maker</p>
              <p class="address">kujira52...op32d</p>
            </div>
            <div class="separator">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 12H19" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                <path
                  d="M12 5L19 12L12 19"
                  stroke="inherit"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            </div>
            <div class="peer-wrap">
              <p class="peer">Taker</p>
              <p class="address">kujira52...op32d</p>
            </div>
          </div>
          <div class="trade-info">
            <p class="label">Transaction summary</p>
            <div class="transaction">
              <div class="list-item">
                <p v-if="isBuyer" class="list-item-label">You will get</p>
                <p v-else class="list-item-label">You will send</p>
                <p class="value">{{ formatAmount(trade.amount) }} {{ microDenomToDenom(trade.denom.native) }}</p>
              </div>
              <div class="list-item">
                <p v-if="isBuyer" class="list-item-label">You will send</p>
                <p v-else class="list-item-label">You will get</p>
                <p class="value fiat">
                  {{ fiatAmountStr }}
                </p>
              </div>
            </div>
          </div>
        </div> -->
        <!-- End Trade Dispute Summary -->
        <TradeActions :tradeInfo="tradeInfo" :walletAddress="walletAddress" />
      </div>
    </section>
  </main>
  <main v-else class="page">
    <div v-if="!client.userWallet.isConnected" class="error-state card">
      <p>You need to connect your wallet</p>
    </div>
    <div v-else class="loading-state">
      <Loading />
    </div>
  </main>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

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

  .step-checked {
    opacity: 0.3;
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

.error-state {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

.loading-state {
  display: flex;
  justify-content: center;
  margin-top: 92px;
}

.wrap {
  display: flex;
}

.main-wrap {
  display: flex;
}

.chat {
  width: 30%;
  margin-right: 24px;
  margin-bottom: 64px;
}

.inner-wrap {
  display: flex;
  flex-direction: column;
  width: 70%;
}

.dispute-summary {
  flex-direction: column;

  .dispute-wrap {
    display: flex;
    justify-content: space-around;
    align-items: center;
    margin-bottom: 40px;

    .peer-wrap {
      text-align: center;

      .peer {
        font-size: 20px;
        font-weight: $semi-bold;
        margin-bottom: 8px;
      }
      .address {
        font-size: 14px;
        background-color: $gray300;
        border-radius: 8px;
        padding: 4px 16px;
      }
    }
    .separator svg {
      stroke: $primary;
    }
  }
}

.trade-summary,
.dispute-summary {
  display: flex;

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
</style>
