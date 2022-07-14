<script setup lang="ts">
import CurrencyInput from '../CurrencyInput.vue'
import {
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  scrollToElement,
} from '~/shared'
import { OfferType } from '~/types/components.interface'
import type { GetOffer, NewTrade } from '~/types/components.interface'
import { usePriceStore } from '~/stores/price'
import { useClientStore } from '~/stores/client'

const props = defineProps<{ offer: GetOffer }>()
const priceStore = usePriceStore()
const client = useClientStore()

let refreshRateInterval: NodeJS.Timer | undefined
const secondsUntilRateRefresh = ref(0)
const cryptoAmount = ref(0)
const fiatAmount = ref(0)
const tradingFee = ref(0.0)
const watchingCrypto = ref(true)
const watchingFiat = ref(false)
const expandedCard = ref()
const cryptoAmountInput = ref()
const fiatAmountInput = ref()
const marginRate = computed(() => convertOfferRateToMarginRate(props.offer.rate))

const fromLabel = computed(() => props.offer.offer_type === OfferType.buy ? 'I want to sell' : 'I want to buy')
const toLabel = computed(() => props.offer.offer_type === OfferType.buy ? 'I will receive' : 'I will pay')
const fiatPlaceholder = computed(() => `${props.offer.fiat_currency.toUpperCase()} 0`)
const cryptoPlaceholder = computed(() => `${props.offer.denom.native} ${parseFloat('0').toFixed(2)}`)
const fiatPriceByRate = computed(() => calculateFiatPriceByRate(priceStore.getPrice(props.offer.fiat_currency), props.offer.rate))
const minAmountInCrypto = computed(() => (parseInt(props.offer.min_amount.toString()) / 1000000))
const maxAmountInCrypto = computed(() => (parseInt(props.offer.max_amount.toString()) / 1000000))
const maxAmountInFiat = computed(() => fiatPriceByRate.value * (parseInt(props.offer.max_amount.toString()) / 1000000))
const minAmountInFiat = computed(() => fiatPriceByRate.value * (parseInt(props.offer.min_amount.toString()) / 1000000))
const offerPrice = computed(() => `${props.offer.fiat_currency} ${formatAmount(fiatPriceByRate.value, false)}`)
const valid = computed(() => true)
const minMaxFiatStr = computed(() => {
  const symbol = props.offer.fiat_currency.toUpperCase()
  const min = minAmountInFiat.value.toFixed(2)
  const max = maxAmountInFiat.value.toFixed(2)
  return [`${symbol} ${min}`, `${symbol} ${max}`]
})
const minMaxCryptoStr = computed(() => {
  const symbol = props.offer.denom.native // TODO: get from offer
  const min = (parseInt(props.offer.min_amount.toString()) / 1000000).toFixed(2)
  const max = (parseInt(props.offer.max_amount.toString()) / 1000000).toFixed(2)
  return [`${symbol} ${min}`, `${symbol} ${max}`]
})

function newTrade() {
  const newTrade: NewTrade = {
    offer_id: `${props.offer.id}`,
    amount: `${cryptoAmount.value * 1000000}`,
    taker: `${client.userWallet.address}`,
  }
  client.openTrade(newTrade)
}

function focus() {
  scrollToElement(expandedCard.value)
}

function useMinCrypto() {
  watchingFiat.value = false
  watchingCrypto.value = true
  cryptoAmountInput.value.update(minAmountInCrypto.value)
}

function useMaxCrypto() {
  watchingFiat.value = false
  watchingCrypto.value = true
  cryptoAmountInput.value.update(maxAmountInCrypto.value)
}

function useMinFiat() {
  watchingFiat.value = true
  watchingCrypto.value = false
  fiatAmountInput.value.update(minAmountInFiat.value)
}

function useMaxFiat() {
  watchingFiat.value = true
  watchingCrypto.value = false
  fiatAmountInput.value.update(maxAmountInFiat.value)
}

watch(fiatAmount, (newFiatAmount) => {
  if (watchingFiat && newFiatAmount !== null) {
    const usdRate = fiatPriceByRate.value
    const cryptoAmount = parseFloat(newFiatAmount.toString()) / usdRate
    tradingFee.value = cryptoAmount * 0.01
    nextTick(() => {
      cryptoAmountInput.value.update(cryptoAmount)
    })
  }
})

watch(cryptoAmount, (newCryptoAmount) => {
  if (watchingCrypto && newCryptoAmount !== null) {
    const usdRate = fiatPriceByRate.value
    tradingFee.value = parseFloat(newCryptoAmount.toString()) * 0.01
    nextTick(() => {
      const fiatAmount = parseFloat(newCryptoAmount.toString()) * usdRate
      fiatAmountInput.value.update(fiatAmount)
    })
  }
})

function refreshExchangeRate() {
  priceStore.fetchPrices()
}

function startExchangeRateRefreshTimer() {
  let seconds = 60
  const countdownInterval = 1000
  refreshRateInterval = setInterval(() => {
    secondsUntilRateRefresh.value = --seconds
    if (seconds === 0) {
      refreshExchangeRate()
      seconds = 60
    }
  }, countdownInterval)
}

onMounted(() => {
  startExchangeRateRefreshTimer()
  nextTick(() => {
    focus()
  })
})

onUnmounted(() => {
  clearInterval(refreshRateInterval)
})
</script>

<template>
  <div :key="`${offer.id}-expanded`" ref="expandedCard" class="offer expanded">
    <div class="owner">
      <p class="wallet">
        {{ formatAddress(offer.owner) }}
      </p>
      <p class="n-trades">
        0 trades
      </p>
    </div>

    <div class="divider-horizontal" />

    <div class="offer-detail">
      <div class="wrap-input">
        <div class="input">
          <p class="label">
            {{ fromLabel }}
          </p>
          <CurrencyInput
            ref="cryptoAmountInput"
            v-model="cryptoAmount"
            :placeholder="cryptoPlaceholder"
            :options="{
              currency: 'USD',
              currencyDisplay: 'hidden',
              hideCurrencySymbolOnFocus: false,
              hideGroupingSeparatorOnFocus: false,
              precision: 2,
              valueRange: {
                min: minAmountInCrypto,
                max: maxAmountInCrypto,
              },
            }"
            @focus="watchingCrypto = true; watchingFiat = false;"
          />
          <div class="wrap-limit">
            <div class="limit-btn">
              <p class="btn" @click="useMinCrypto()">
                {{ minMaxCryptoStr[0] }}
              </p>
              <p>-</p>
              <p class="btn" @click="useMaxCrypto()">
                {{ minMaxCryptoStr[1] }}
              </p>
            </div>
          </div>
        </div>

        <div class="input">
          <p class="label">
            {{ toLabel }}
          </p>
          <CurrencyInput
            ref="fiatAmountInput"
            v-model="fiatAmount"
            :placeholder="fiatPlaceholder"
            :options="{
              currency: offer.fiat_currency.toUpperCase(),
              currencyDisplay: 'code',
              hideCurrencySymbolOnFocus: false,
              hideGroupingSeparatorOnFocus: false,
              precision: 2,
              valueRange: {
                min: minAmountInFiat,
                max: maxAmountInFiat,
              },
            }"
            @focus="
              watchingCrypto = false;
              watchingFiat = true;
            "
          />

          <div class="wrap-limit">
            <div class="limit-btn">
              <p class="btn" @click="useMinFiat()">
                {{ minMaxFiatStr[0] }}
              </p>
              <p>-</p>
              <p class="btn" @click="useMaxFiat()">
                {{ minMaxFiatStr[1] }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <div class="receipt">
        <div class="price">
          <p class="label">
            Price
          </p>
          <div class="wrap">
            <p class="ticker">
              Will refresh in {{ secondsUntilRateRefresh }}s
            </p>
            <p class="margin">
              {{ marginRate.marginOffset }}% {{ marginRate.margin }} market
            </p>
            <p class="value">
              1 {{ offer.denom.native }} = {{ offerPrice }}
            </p>
          </div>
        </div>

        <div class="summary">
          <div class="wrap">
            <div class="item">
              <p class="info">
                Trading Fee
              </p>
              <p>{{ tradingFee.toFixed(2) }}</p>
            </div>
            <div class="item">
              <p class="info">
                Total
              </p>
              <p class="total">
                ??????
              </p>
            </div>
          </div>
        </div>

        <div class="wrap-btns">
          <button class="secondary" @click="$emit('cancel')">
            cancel
          </button>
          <button class="primary" :disabled="!valid" @click="newTrade()">
            open trade
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.owner {

  .wallet {
    font-size: 18px;
    font-weight: 600;
    color: $base-text;
  }

  .n-trades {
    font-size: 14px;
    color: $gray600;
  }

  @media only screen and (max-width: $mobile) {
    .owner {
      display: inline-flex;
    }
  }
}

.expanded {
  display: flex;
  flex-direction: column;
  gap: 16px;

  .divider-horizontal {
    width: 100%;
    height: 1px;
    background-color: $border;
    margin: 8px 0;
  }

  .offer-detail {
    display: flex;
    justify-content: space-between;
    gap: 80px;

    @media only screen and (max-width: $mobile) {
      flex-direction: column;
      gap: 0px;
    }

    .wrap-input {
      min-width: 350px;

      @media only screen and (max-width: $mobile) {
        width: 100%;
        min-width: 0;
      }

      .input {
        margin-bottom: 24px;
      }

      input {
        color: $base-text;
        background-color: $background;
        text-align: right;
      }

      .label {
        font-size: 14px;
        color: $gray600;
        margin-bottom: 8px;
      }

      .wrap-limit {
        display: flex;
        justify-content: flex-end;

        .limit-btn {
          display: flex;
          gap: 4px;

          p {
            font-size: 12px;
            color: $gray600;
            text-align: right;
            margin-top: 8px;
          }
          .btn {
              text-decoration: underline;
              cursor: pointer;
            }
        }

      }
    }

    .receipt {
      width: 100%;

       @media only screen and (max-width: $mobile) {
            border-top: 1px solid $border;
            padding-top: 24px;
            margin-top: 8px;
          }

      .price {
        margin-bottom: 24px;

        .label {
          font-size: 14px;
          color: $gray600;
        }

        .wrap {
          width: 100%;
          display: flex;
          justify-content: space-between;
          background-color: $gray150;
          border-radius: 8px;
          padding: 10px 24px;
          margin-top: 8px;
          align-items: center;
          gap: 16px;

          @media only screen and (max-width: $mobile) {
            flex-direction: column;
          }

          .ticker {
            font-size: 12px;
            color: $primary;
          }

          .margin {
            font-size: 14px;
            color: $gray600;
          }

          .value {
            font-size: 16px;
            color: $base-text;
          }
        }
      }

      .summary {
        margin-bottom: 24px;

        .label {
          font-size: 14px;
          color: $gray600;
        }

        .wrap {
          width: 100%;
          display: flex;
          flex-direction: column;
          justify-content: space-between;
          background-color: $gray150;
          border-radius: 8px;
          padding: 16px 24px;
          margin-top: 8px;

          gap: 8px;

          .item {
            display: inline-flex;
            justify-content: space-between;

            .info {
              color: $gray700;
            }

            .price-get {
              font-weight: 800;
            }

            .total {
              color: $primary;
              font-weight: 600;
            }
          }
        }
      }

      .wrap-btns {
        display: flex;
        justify-content: flex-end;
        gap: 24px;
      }
    }
  }

}
</style>
