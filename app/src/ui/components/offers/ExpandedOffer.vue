<script setup lang="ts">
import {
  addTelegramURLPrefix,
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  formatTradesCountInfo,
  removeTelegramURLPrefix,
  scrollToElement,
} from '~/shared'
import { OfferType } from '~/types/components.interface'
import type { NewTrade, OfferResponse } from '~/types/components.interface'
import { usePriceStore } from '~/stores/price'
import { useClientStore } from '~/stores/client'
import { microDenomToDenom } from '~/utils/denom'
import { decryptData, encryptData } from '~/utils/crypto'

const props = defineProps<{ offerResponse: OfferResponse }>()
const emit = defineEmits<{ (e: 'cancel'): void }>()
const priceStore = usePriceStore()
const client = useClientStore()
const secrets = computed(() => client.getSecrets())

async function defaultTakerContact() {
  const contact = client.profile?.contact
  console.log('contact: ', contact)
  if (contact !== undefined) {
    const decryptedContact = await decryptData(secrets.value.privateKey, contact)
    return addTelegramURLPrefix(decryptedContact)
  } else {
    return ''
  }
}

let refreshRateInterval: NodeJS.Timer | undefined
const telegram = ref<string>('')
const secondsUntilRateRefresh = ref(0)
const cryptoAmount = ref(0)
const fiatAmount = ref(0)
const tradingFee = ref(0.0)
const watchingCrypto = ref(true)
const watchingFiat = ref(false)
const expandedCard = ref()
const cryptoAmountInput = ref()
const fiatAmountInput = ref()
const marginRate = computed(() => convertOfferRateToMarginRate(props.offerResponse.offer.rate))

const fromLabel = computed(() =>
  props.offerResponse.offer.offer_type === OfferType.buy ? 'I want to sell' : 'I want to buy'
)
const toLabel = computed(() =>
  props.offerResponse.offer.offer_type === OfferType.buy ? 'I will receive' : 'I will pay'
)
const fiatPlaceholder = computed(() => `${props.offerResponse.offer.fiat_currency.toUpperCase()} 0`)
const cryptoPlaceholder = computed(
  () => `${microDenomToDenom(props.offerResponse.offer.denom.native)} ${parseFloat('0').toFixed(2)}`
)
const fiatPriceByRate = computed(() =>
  calculateFiatPriceByRate(priceStore.getPrice(props.offerResponse.offer.fiat_currency), props.offerResponse.offer.rate)
)
const minAmountInCrypto = computed(() => parseInt(props.offerResponse.offer.min_amount.toString()) / 1000000)
const maxAmountInCrypto = computed(() => parseInt(props.offerResponse.offer.max_amount.toString()) / 1000000)
const maxAmountInFiat = computed(
  () => fiatPriceByRate.value * (parseInt(props.offerResponse.offer.max_amount.toString()) / 1000000)
)
const minAmountInFiat = computed(
  () => fiatPriceByRate.value * (parseInt(props.offerResponse.offer.min_amount.toString()) / 1000000)
)
const offerPrice = computed(
  () => `${props.offerResponse.offer.fiat_currency} ${formatAmount(fiatPriceByRate.value, false)}`
)
const valid = computed(() => true)
const minMaxFiatStr = computed(() => {
  const symbol = props.offerResponse.offer.fiat_currency.toUpperCase()
  const min = minAmountInFiat.value.toFixed(2)
  const max = maxAmountInFiat.value.toFixed(2)
  return [`${symbol} ${min}`, `${symbol} ${max}`]
})
const minMaxCryptoStr = computed(() => {
  const symbol = microDenomToDenom(props.offerResponse.offer.denom.native)
  const min = (parseInt(props.offerResponse.offer.min_amount.toString()) / 1000000).toFixed(2)
  const max = (parseInt(props.offerResponse.offer.max_amount.toString()) / 1000000).toFixed(2)
  return [`${symbol} ${min}`, `${symbol} ${max}`]
})

async function newTrade() {
  const telegramHandle = removeTelegramURLPrefix(telegram.value) as string
  const profile_taker_encryption_key = secrets.value.publicKey
  const taker_contact = await encryptData(props.offerResponse.profile.encryption_key!, telegramHandle)
  const profile_taker_contact = await encryptData(profile_taker_encryption_key, telegramHandle)

  const newTrade: NewTrade = {
    offer_id: `${props.offerResponse.offer.id}`,
    amount: `${cryptoAmount.value * 1000000}`,
    taker: `${client.userWallet.address}`,
    profile_taker_contact,
    taker_contact,
    profile_taker_encryption_key,
  }
  await client.openTrade(newTrade)
}

function focus() {
  scrollToElement(expandedCard.value)
}

function toggleCryptoFiat() {
  watchingCrypto.value = !watchingCrypto.value
  watchingFiat.value = !watchingFiat.value
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
  nextTick(async () => {
    telegram.value = await defaultTakerContact()
    focus()
  })
})

onUnmounted(() => {
  clearInterval(refreshRateInterval)
})
</script>

<template>
  <div :key="`${offerResponse.offer.id}-expanded`" ref="expandedCard" class="offer expanded">
    <div class="owner">
      <p class="wallet">
        {{ formatAddress(offerResponse.offer.owner) }}
      </p>
      <p class="n-trades">{{ formatTradesCountInfo(offerResponse.offer.trades_count) }}</p>
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
            @focus="
              (() => {
                watchingCrypto = true
                watchingFiat = false
              })()
            "
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
              currency: offerResponse.offer.fiat_currency.toUpperCase(),
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
              (() => {
                watchingCrypto = false
                watchingFiat = true
              })()
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

        <div class="telegram">
          <div class="wrap-label">
            <p class="label">Your Telegram username</p>
            <IconTooltip
              content="Share your contact to be able to communicate with the other trader. This information will be encrypted and only visible inside the trade."
            />
          </div>
          <input v-model="telegram" type="text" placeholder="t.me/your-user-name" />
        </div>
      </div>

      <div class="receipt">
        <div class="price">
          <p class="label">Price</p>
          <div class="wrap">
            <p class="margin">{{ marginRate.marginOffset }}% {{ marginRate.margin }} market</p>
            <p class="value">1 {{ microDenomToDenom(offerResponse.offer.denom.native) }} = {{ offerPrice }}</p>
            <p class="ticker">Will refresh in {{ secondsUntilRateRefresh }}s</p>
          </div>
        </div>

        <div class="summary">
          <p class="label">Transaction summary</p>
          <div class="wrap">
            <div class="item">
              <p class="info">Trading Fee</p>
              <p>{{ microDenomToDenom(offerResponse.offer.denom.native) }} {{ tradingFee.toFixed(2) }}</p>
            </div>
            <div class="item">
              <p class="info">Total</p>
              <p class="total">??????</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="wrap-btns">
      <button class="secondary" @click="emit('cancel')">cancel</button>
      <button class="primary bg-gray300" :disabled="!valid" @click="newTrade()">open trade</button>
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
    display: flex;
    justify-content: space-between;
    align-items: center;
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
    flex-direction: column;
    justify-content: space-between;
    gap: 32px;

    @media only screen and (max-width: $mobile) {
      flex-direction: column;
      gap: 0px;
    }

    .label {
      font-size: 14px;
      color: $gray900;
    }

    .wrap-input {
      display: flex;
      justify-content: space-between;
      gap: 24px;

      @media only screen and (max-width: $mobile) {
        width: 100%;
        min-width: 0;
      }

      .input {
        flex: 2;
        margin-bottom: 8px;

        input {
          margin-top: 8px;
          color: $base-text;
          background-color: $background;
          text-align: right;
        }
      }

      .telegram {
        flex: 3;
        margin-bottom: 8px;

        .wrap-label {
          display: flex;
          gap: 8px;
        }

        input {
          margin-top: 8px;
          color: $base-text;
          background-color: $background;
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

    .receipt {
      display: flex;
      justify-content: space-between;
      gap: 24px;
      width: 100%;

      @media only screen and (max-width: $mobile) {
        border-top: 1px solid $border;
        padding-top: 24px;
        margin-top: 24px;
      }
      .price {
        flex: 2;
        .wrap {
          width: 100%;
          display: flex;
          justify-content: space-between;
          flex-wrap: wrap;
          background-color: $gray150;
          padding: 16px 24px;
          margin-top: 8px;
          border-radius: 8px;
          align-items: center;
          gap: 16px;

          @media only screen and (max-width: $mobile) {
            flex-direction: column;
            gap: 4px;
            padding: 16px 24px;
          }

          .ticker {
            width: 100%;
            text-align: center;
            font-size: 12px;
            color: $primary;
          }

          .margin {
            font-size: 14px;
            color: $gray700;
          }

          .value {
            font-size: 16px;
            color: $base-text;
          }
        }
      }

      .summary {
        flex: 3;

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
    }
  }
  .wrap-btns {
    display: flex;
    justify-content: flex-end;
    gap: 24px;
    margin-top: 8px;
  }
}
</style>
