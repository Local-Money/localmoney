<script setup lang="ts">
import {
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  formatEncryptedUserContact,
  formatTradesCountInfo,
  isTelegramHandleValid,
  removeTelegramHandlePrefix,
  scrollToElement,
} from '~/shared'
import { OfferType } from '~/types/components.interface'
import type { OfferResponse } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { denomToValue, microDenomToDisplay } from '~/utils/denom'
import { formatTimeLimit } from '~/utils/formatters'
import { CRYPTO_DECIMAL_PLACES, FIAT_DECIMAL_PLACES } from '~/utils/constants'

const props = defineProps<{ offerResponse: OfferResponse }>()
const emit = defineEmits<{ (e: 'cancel'): void }>()
const client = useClientStore()
const secrets = computed(() => client.getSecrets())

async function defaultUserContact() {
  const contact = client.profile?.contact
  if (secrets.value) {
    return formatEncryptedUserContact(secrets.value.privateKey, contact)
  } else {
    return ''
  }
}

let refreshRateInterval: NodeJS.Timer | undefined
const telegram = ref<string>('')
const secondsUntilRateRefresh = ref(0)
const cryptoAmount = ref(0.0)
const fiatAmount = ref(0.0)
const fiatPriceByRate = ref(0.0)
const expandedCard = ref()
const cryptoAmountInput = ref()
const fiatAmountInput = ref()
const marginRate = computed(() => convertOfferRateToMarginRate(props.offerResponse.offer.rate))
const watchingFiat = ref(false)
const watchingCrypto = ref(false)

const tradeTimeLimit = computed(() => {
  const expirationTime = client.getHubConfig().trade_expiration_timer * 1000
  const time = new Date(expirationTime)
  return formatTimeLimit(time)
})

const fromLabel = computed(() =>
  props.offerResponse.offer.offer_type === OfferType.buy ? 'I want to sell' : 'I want to buy'
)
const toLabel = computed(() =>
  props.offerResponse.offer.offer_type === OfferType.buy ? 'I will receive' : 'I will pay'
)
const fiatPlaceholder = computed(() => `${props.offerResponse.offer.fiat_currency.toUpperCase()} 0`)
const cryptoPlaceholder = computed(
  () =>
    `${microDenomToDisplay(denomToValue(props.offerResponse.offer.denom), client.chainClient)} ${parseFloat(
      '0'
    ).toFixed(2)}`
)
const minAmountInCrypto = computed(
  () => parseInt(props.offerResponse.offer.min_amount.toString()) / CRYPTO_DECIMAL_PLACES
)
const maxAmountInCrypto = computed(
  () => parseInt(props.offerResponse.offer.max_amount.toString()) / CRYPTO_DECIMAL_PLACES
)
const maxAmountInFiat = computed(() =>
  parseFloat(
    (fiatPriceByRate.value * (parseInt(props.offerResponse.offer.max_amount.toString()) / FIAT_DECIMAL_PLACES)).toFixed(
      2
    )
  )
)
const minAmountInFiat = computed(() =>
  parseFloat(
    (fiatPriceByRate.value * (parseInt(props.offerResponse.offer.min_amount.toString()) / FIAT_DECIMAL_PLACES)).toFixed(
      2
    )
  )
)
const offerPrice = computed(
  () => `${props.offerResponse.offer.fiat_currency} ${formatAmount(fiatPriceByRate.value / 100, false)}`
)
const valid = computed(
  () =>
    cryptoAmount.value >= minAmountInCrypto.value &&
    cryptoAmount.value <= maxAmountInCrypto.value &&
    isTelegramHandleValid(telegram.value)
)
const minMaxFiatStr = computed(() => {
  const symbol = props.offerResponse.offer.fiat_currency.toUpperCase()
  const min = minAmountInFiat.value.toFixed(2)
  const max = maxAmountInFiat.value.toFixed(2)
  return [`${symbol} ${min}`, `${symbol} ${max}`]
})
const minMaxCryptoStr = computed(() => {
  const symbol = microDenomToDisplay(denomToValue(props.offerResponse.offer.denom), client.chainClient)
  const min = formatAmount(parseInt(props.offerResponse.offer.min_amount), true, 6)
  const max = formatAmount(parseInt(props.offerResponse.offer.max_amount), true, 6)
  return [`${symbol} ${parseFloat(min)}`, `${symbol} ${parseFloat(max)}`]
})

async function newTrade() {
  const telegramHandle = removeTelegramHandlePrefix(telegram.value) as string
  await client.openTrade(props.offerResponse, telegramHandle, cryptoAmount.value)
}

function focus() {
  scrollToElement(expandedCard.value)
}

function useMinCrypto() {
  cryptoAmount.value = minAmountInCrypto.value
  const usdRate = fiatPriceByRate.value / 100
  fiatAmount.value = parseFloat((cryptoAmount.value * usdRate).toFixed(2))
}

function useMaxCrypto(fromFiat = false) {
  cryptoAmount.value = maxAmountInCrypto.value
  const usdRate = fiatPriceByRate.value / 100
  fiatAmount.value = parseFloat((cryptoAmount.value * usdRate).toFixed(2))
}

watch(fiatAmount, (newFiatAmount) => {
  if (watchingCrypto.value) {
    return
  }
  if (newFiatAmount === maxAmountInFiat.value) {
    useMaxCrypto()
  } else if (newFiatAmount === minAmountInFiat.value) {
    useMinCrypto()
  } else {
    if (newFiatAmount === 0 || isNaN(newFiatAmount)) {
      cryptoAmount.value = 0.0
    } else {
      const usdRate = fiatPriceByRate.value / 100
      cryptoAmount.value = parseFloat((newFiatAmount / usdRate).toFixed(6))
    }
  }
})

watch(cryptoAmount, (newCryptoAmount) => {
  if (watchingFiat.value) {
    return
  }
  if (newCryptoAmount === 0 || isNaN(newCryptoAmount)) {
    fiatAmount.value = 0.0
  } else {
    const usdRate = fiatPriceByRate.value / 100
    fiatAmount.value = parseFloat((newCryptoAmount * usdRate).toFixed(2))
  }
})

async function refreshExchangeRate() {
  const offer = props.offerResponse.offer
  const denomFiatPrice = await client.fetchFiatPriceForDenom(offer.fiat_currency, offer.denom)
  const price = calculateFiatPriceByRate(denomFiatPrice.price, props.offerResponse.offer.rate)
  fiatPriceByRate.value = price
  fiatAmount.value = parseFloat(cryptoAmount.value.toString()) * (fiatPriceByRate.value / 100)
}

function startExchangeRateRefreshTimer() {
  const interval = 60
  let seconds = interval
  const countdownInterval = 1000
  refreshRateInterval = setInterval(async () => {
    secondsUntilRateRefresh.value = --seconds
    if (seconds === 0) {
      await refreshExchangeRate()
      seconds = interval
    }
  }, countdownInterval)
}

function focusFiat() {
  watchingFiat.value = true
  watchingCrypto.value = false
}

function focusCrypto() {
  watchingFiat.value = false
  watchingCrypto.value = true
}

onMounted(async () => {
  const denomFiatPrice = client.fiatPrices
    .get(props.offerResponse.offer.fiat_currency)
    ?.get(denomToValue(props.offerResponse.offer.denom))
  const price = calculateFiatPriceByRate(denomFiatPrice, props.offerResponse.offer.rate)
  fiatPriceByRate.value = price
  startExchangeRateRefreshTimer()
  telegram.value = await defaultUserContact()
  nextTick(async () => {
    focus()
  })
})

const tradeCountIcon = computed(() => props.offerResponse.profile.released_trades_count > 0)

onUnmounted(() => {
  clearInterval(refreshRateInterval)
})
</script>

<template>
  <div :key="`${offerResponse.offer.id}-expanded`" ref="expandedCard" class="offer expanded card">
    <div class="top">
      <div class="owner">
        <a class="wallet-addr" :href="`/maker/${offerResponse.offer.owner}`">
          {{ formatAddress(offerResponse.offer.owner) }}
        </a>
        <div class="n-trades">
          <svg v-show="tradeCountIcon" class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none">
            <path
              d="M20 6L9 17L4 12"
              stroke="inherit"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          <p>{{ formatTradesCountInfo(offerResponse.profile.released_trades_count) }}</p>
        </div>
      </div>

      <div class="inner-wrap">
        <div class="description">
          <p class="content">{{ offerResponse.offer.description ?? 'No Description' }}</p>
        </div>
        <div class="price">
          <div class="wrap">
            <p class="value">
              1 {{ microDenomToDisplay(offerResponse.offer.denom.native, client.chainClient) }} = {{ offerPrice }}
            </p>
          </div>
          <p class="ticker">refresh in {{ secondsUntilRateRefresh }}s</p>
        </div>
      </div>
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
            :prefix="microDenomToDisplay(offerResponse.offer.denom.native, client.chainClient)"
            :min="minAmountInCrypto"
            :max="maxAmountInCrypto"
            :errorMsg="`The value should be between ${minMaxCryptoStr[0]} and ${minMaxCryptoStr[1]}`"
            :isCrypto="true"
            :decimals="6"
            @focus="focusCrypto"
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
            :prefix="offerResponse.offer.fiat_currency"
            :min="minAmountInFiat"
            :max="maxAmountInFiat"
            :errorMsg="`The value should be between ${minMaxFiatStr[0]} and ${minMaxFiatStr[1]}`"
            :isCrypto="false"
            :decimals="2"
            @focus="focusFiat"
          />
          <div class="wrap-limit">
            <div class="limit-btn">
              <p class="btn" @click="useMinCrypto()">
                {{ minMaxFiatStr[0] }}
              </p>
              <p>-</p>
              <p class="btn" @click="useMaxCrypto()">
                {{ minMaxFiatStr[1] }}
              </p>
            </div>
          </div>
        </div>

        <div class="telegram">
          <div class="wrap-label">
            <p class="label">Your Telegram username</p>
            <IconTooltip
              content="Share your contact to be able to communicate with the other trader. This information will be encrypted and only visible inside the trade."
            />
          </div>
          <input v-model="telegram" type="text" placeholder="@username" />
        </div>
      </div>
    </div>

    <footer>
      <div class="time-limit">
        <p class="label">Trade time limit</p>
        <p class="value">{{ tradeTimeLimit }}</p>
      </div>
      <div class="wrap-btns">
        <button class="secondary" @click="emit('cancel')">cancel</button>
        <button class="primary bg-gray300" :disabled="!valid" @click="newTrade()">open trade</button>
      </div>
    </footer>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.expanded {
  display: flex;
  flex-direction: column;
  gap: 16px;

  .top {
    display: flex;
    justify-content: space-between;

    @include responsive(mobile) {
      flex-direction: column;
    }

    .owner {
      width: 20%;
      display: flex;
      flex-direction: column;

      @include responsive(mobile) {
        width: 100%;
        margin-bottom: 24px;
        flex-direction: row;
      }
      .wallet-addr {
        font-size: 16px;
        font-weight: 600;
        color: $base-text;
        &:hover {
          color: $primary;
        }
      }

      .n-trades {
        display: flex;
        align-items: center;
        align-self: flex-start;
        gap: 6px;
        margin-top: 8px;

        background-color: $border;
        padding: 4px 8px;
        border-radius: 8px;

        @include responsive(mobile) {
          margin-top: 0;
        }

        svg {
          width: 16px;
          height: 16px;
          stroke: $primary;
        }

        p {
          font-size: 12px;
          color: $gray700;
          padding-right: 4px;
        }
      }

      @media only screen and (max-width: $mobile) {
        display: flex;
        justify-content: space-between;
        align-items: center;
      }
    }

    .divider {
      height: 40px;
      width: 1px;
      background-color: $border;
    }

    .inner-wrap {
      width: 80%;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 32px;
      padding-left: 32px;
      border-left: 1px solid $border;

      @include responsive(mobile) {
        width: 100%;
        display: flex;
        flex-direction: column-reverse;
        justify-content: space-between;
        gap: 24px;
        padding: 0;
        border: none;
      }
      .price {
        width: 50%;
        display: flex;
        flex-direction: row-reverse;
        gap: 32px;
        flex-shrink: 0;
        align-items: center;

        @include responsive(mobile) {
          width: 100%;
          flex-direction: row-reverse;
          justify-content: space-between;
          gap: 16px;
        }

        .ticker {
          text-align: center;
          font-size: 12px;
          color: $gray900;
          background-color: $border;
          padding: 4px 12px;
          border-radius: 8px;
        }

        .wrap {
          text-align: right;
          @include responsive(mobile) {
            text-align: right;
          }
        }

        .margin {
          font-size: 14px;
          color: $gray700;
          @include responsive(mobile) {
            font-size: 12px;
          }
        }

        .value {
          font-size: 16px;
          color: $base-text;
          @include responsive(mobile) {
            font-size: 14px;
          }
        }
      }
      .description {
        @include responsive(mobile) {
          background-color: $gray150;
          border-radius: 8px;
          padding: 12px 16px;
        }
        .content {
          font-size: 14px;
          color: $gray900;
          @include responsive(mobile) {
            font-size: 12px;
          }
        }
      }
    }
  }

  .divider-horizontal {
    width: 100%;
    height: 1px;
    background-color: $border;
    margin: 8px 0;
  }

  .offer-detail {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 32px;

    .label {
      font-size: 14px;
      color: $gray900;
    }

    .wrap-input {
      display: flex;
      justify-content: space-between;
      gap: 24px;

      @include responsive(mobile) {
        flex-direction: column;
        gap: 8px;
      }

      .input,
      .telegram {
        margin-bottom: 8px;

        @include responsive(mobile) {
          margin-bottom: 0;
        }

        input {
          margin-top: 8px;
          color: $base-text;
          background-color: $background;
        }

        .wrap-label {
          display: flex;
          gap: 8px;
        }
      }

      .input {
        flex: 1;

        input {
          text-align: right;
        }
      }

      .telegram {
        flex: 2;

        @include responsive(mobile) {
          margin-top: 16px;
        }

        input {
          text-align: left;
        }
      }
      .wrap-limit {
        display: flex;
        justify-content: flex-end;
        margin-right: 16px;

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
  }

  footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 16px;

    @include responsive(mobile) {
      flex-direction: column;
      margin-top: 8px;
    }

    .time-limit {
      @include responsive(mobile) {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 24px;
      }
      .label {
        font-size: 14px;
        color: $gray700;
        @include responsive(mobile) {
          font-size: 12px;
        }
      }
      .value {
        font-size: 14px;
        color: $gray900;
        @include responsive(mobile) {
          font-size: 14px;
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
</style>
