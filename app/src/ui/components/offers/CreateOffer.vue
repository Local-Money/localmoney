<script setup lang="ts">
import CurrencyInput from '../CurrencyInput.vue'
import {
  calculateFiatPriceByRate,
  convertMarginRateToOfferRate,
  formatAmount,
} from '~/shared'
import { usePriceStore } from '~/stores/price'
import type { PostOffer } from '~/types/components.interface'
import { FiatCurrency, OfferType } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import {
  defaultMicroDenomAvailable,
  denomsAvailable,
  microDenomToDenom,
} from '~/utils/denom'
import { fiatsAvailable, getFiatInfo } from '~/utils/fiat'

const emit = defineEmits<{
  (e: 'cancel'): void
}>()
const client = useClientStore()
const priceStore = usePriceStore()

const selectedCrypto = ref(defaultMicroDenomAvailable())
const minAmount = ref(0)
const maxAmount = ref(0)
const margin = ref('above')
const marginOffset = ref('')
const marginOffsetUnmasked = ref(0)
const rate = ref(0)
const offerType = ref<OfferType>(OfferType.buy)
const fiatCurrency = ref<FiatCurrency>(FiatCurrency.ARS)
const valid = computed(() => maxAmount.value > minAmount.value)
const usdRate = computed(() => priceStore.getPrice(fiatCurrency.value))
const offerPrice = computed(() => {
  const fiatPrice = calculateFiatPriceByRate(usdRate.value, rate.value)
  return `${fiatCurrency.value} ${formatAmount(fiatPrice, false)}`
})
const fiatLabel = computed(() =>
  offerType.value === 'sell' ? 'receive' : 'pay',
)

// TODO - Make isMobile global
const width = ref(window.innerWidth)
const listener = () => { width.value = window.innerWidth }
onMounted(() => { window.addEventListener('resize', listener) })
onUnmounted(() => { window.removeEventListener('resize', listener) })
const isMobile = computed(() => width.value <= 550)

// Get the viewport height and store in a variable
const vh = window.innerHeight * 0.01
document.documentElement.style.setProperty('--vh', `${vh}px`)

function calculateMarginRate() {
  rate.value = convertMarginRateToOfferRate(
    margin.value,
    marginOffsetUnmasked.value,
  )
}
function createOffer() {
  const postOffer: PostOffer = {
    offer_type: offerType.value,
    fiat_currency: fiatCurrency.value,
    rate: `${rate.value}`,
    denom: { native: selectedCrypto.value },
    min_amount: `${minAmount.value * 1000000}`,
    max_amount: `${maxAmount.value * 1000000}`,
    maker_contact: 'NoContactProvided',
  }
  client.createOffer(postOffer)
  emit('cancel')
}
watch(marginOffset, () => {
  calculateMarginRate()
})
watch(margin, () => {
  calculateMarginRate()
})
</script>

<template>
  <div class="main-wrap card">
    <div class="header-wrap">
      <p>Create Offer</p>
      <div v-if="isMobile" class="close" @click="$emit('cancel')">
        X
      </div>
    </div>
    <div class="buy-sell">
      <button
        :class="{ focus: offerType === 'buy' }"
        @click="offerType = 'buy'"
      >
        Buy
      </button>
      <div class="separator" />
      <button
        :class="{ focus: offerType === 'sell' }"
        @click="offerType = 'sell'"
      >
        Sell
      </button>
    </div>

    <div class="inner-content">
      <div class="currency">
        <div class="wrap">
          <label for="crypto">I want to {{ offerType }}</label>
          <CustomSelect v-model="selectedCrypto" :options="denomsAvailable" />
        </div>
        <div class="wrap">
          <label for="currency">and {{ fiatLabel }} in</label>
          <CustomSelect v-model="fiatCurrency" :options="fiatsAvailable" />
        </div>
      </div>
      <div class="divider" />
      <div class="min-max">
        <div class="wrap">
          <label>Min amount of {{ microDenomToDenom(selectedCrypto) }}</label>
          <CurrencyInput
            v-model="minAmount"
            :placeholder="0"
            :options="{
              currency: 'USD',
              currencyDisplay: 'hidden',
              hideCurrencySymbolOnFocus: false,
              hideGroupingSeparatorOnFocus: false,
              precision: 2,
            }"
          />
        </div>
        <div class="wrap">
          <label>Max amount of {{ microDenomToDenom(selectedCrypto) }}</label>
          <CurrencyInput
            v-model="maxAmount"
            :placeholder="0"
            :options="{
              currency: 'USD',
              currencyDisplay: 'hidden',
              hideCurrencySymbolOnFocus: false,
              hideGroupingSeparatorOnFocus: false,
              precision: 2,
            }"
          />
        </div>
      </div>
      <div class="market-price">
        <div class="wrap">
          <label for="">Market price</label>
          <select v-model="margin" class="bg-surface">
            <option value="above">
              Above
            </option>
            <option value="below">
              Below
            </option>
          </select>
        </div>
        <div class="wrap">
          <label for="currency">Margin Offset</label>
          <input
            v-model="marginOffset"
            v-maska="['##%', '#%']"
            type="text"
            placeholder="0%"
            @maska="marginOffsetUnmasked = $event.target.dataset.maskRawValue"
          >
        </div>
      </div>
      <div class="divider" />
      <div class="chat">
        <div class="wrap">
          <label for="crypto">Your Telegram handle so traders can reach you</label>
          <input type="text">
        </div>
      </div>
    </div>

    <div class="divider" />

    <div class="footer">
      <div class="fiat-price">
        <p class="value">
          1 {{ microDenomToDenom(selectedCrypto) }} = {{ offerPrice }}
        </p>
      </div>
      <div class="btns">
        <button class="secondary" @click="$emit('cancel')">
          Cancel
        </button>
        <button class="primary" :disabled="!valid" @click="createOffer()">
          Create
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";
@import "../../style/elements.scss";

.main-wrap {
  display: inline-flex;
  flex-direction: column;

  @media only screen and (max-width: $mobile) {
    height: 300px;

    overflow: scroll;
  }
}

.header-wrap {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.buy-sell {
    display: flex;
    margin: 24px 0 24px;
  }

.inner-content {
  .currency,
  .min-max,
  .market-price, .chat {
    display: flex;
    gap: 24px;

    .wrap {
      display: flex;
      flex-direction: column;
      width: 100%;

      label {
        font-size: 14px;
        font-weight: 400;
        color: $gray900;
        margin-bottom: 8px;
      }

      input {
        width: 100%;
        background-color: $background;
      }
    }
  }

  .currency {
    @media only screen and (max-width: $mobile) {
      flex-direction: column;
    }
  }
}

.divider {
  width: 100%;
  height: 1px;
  background-color: $border;
  margin: 32px 0;
}

.min-max {
  margin-bottom: 24px;
}

.footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 24px;

  .fiat-price {
    @media only screen and (max-width: $mobile) {
      font-size: 12px;
    }
  }
}

.btns {
  display: flex;
  justify-content: flex-end;
  gap: 16px;
  margin-top: 0px;
}
</style>
