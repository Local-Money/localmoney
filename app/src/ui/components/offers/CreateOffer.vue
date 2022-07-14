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

const emit = defineEmits<{
  (e: 'cancel'): void
}>()
const client = useClientStore()
const priceStore = usePriceStore()

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

function calculateMarginRate() {
  rate.value = convertMarginRateToOfferRate(margin.value, marginOffsetUnmasked.value)
}
function createOffer() {
  const postOffer: PostOffer = {
    offer_type: offerType.value,
    fiat_currency: fiatCurrency.value,
    rate: `${rate.value}`,
    // TODO remove this hard coded denom
    denom: { native: 'ujunox' },
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
  <div class="main-wrap">
    <p>Create Offer</p>
    <div class="header-wrap">
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
      <div class="price">
        <p class="value">
          {{ offerPrice }}
        </p>
      </div>
    </div>

    <div class="card">
      <div class="currency">
        <div class="filter">
          <label for="crypto">Crypto</label>
          <select id="crypto" class="bg-gray300" name="crypto">
            <option value="KUJI">
              KUJI
            </option>
          </select>
        </div>
        <div class="filter">
          <label for="currency">Currency (FIAT)</label>
          <select
            id="currency"
            v-model="fiatCurrency"
            class="bg-gray300"
            name="currency"
          >
            <option :value="FiatCurrency.ARS">
              ARS
            </option>
            <option :value="FiatCurrency.BRL">
              BRL
            </option>
            <option :value="FiatCurrency.COP">
              COP
            </option>
          </select>
        </div>
      </div>
      <div class="divider" />
      <div class="wrap-price">
        <div class="margin">
          <label for="">Margin</label>
          <select v-model="margin" class="bg-gray300">
            <option value="above">
              Above
            </option>
            <option value="below">
              Below
            </option>
          </select>
        </div>
        <div class="margin-offset">
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

      <div class="min-max">
        <div class="wrap">
          <label>Min amount:</label>
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
          <label>Max amount:</label>
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
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";
@import "../../style/elements.scss";

.main-wrap {
    display: inline-flex;
    flex-direction: column;
}

.buy-sell {
    display: flex;
    margin: 24px 0 24px;
}

.header-wrap {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .value {
        font-size: 16px;
        color: $base-text;
        font-weight: $semi-bold;
    }
}

.divider {
    width: 100%;
    height: 1px;
    background-color: $border;
    margin: 32px 0;
}

.wrap-price {
    display: flex;
    justify-items: center;
    align-content: center;
    gap: 24px;
    margin-bottom: 24px;

    .margin,
    .margin-offset {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 8px;

        label {
            font-size: 14px;
            font-weight: 400;
            color: $gray900;
        }
    }

    input {
        width: 100%;
        background-color: $background;
    }
}

.min-max {
    display: inline-flex;
    flex-basis: content;

    .wrap {
        display: flex;
        flex-direction: column;

        &:last-child {
            margin-left: 24px;
        }

        label {
            font-size: 14px;
            font-weight: 400;
            color: $gray900;
            margin-bottom: 8px;
        }
    }

    input {
        width: 100%;
        background-color: $background;
    }
}

.btns {
    display: flex;
    justify-content: flex-end;
    gap: 24px;
    margin-top: 24px;
}

.currency {
    display: flex;

    .filter {
        display: flex;
        flex-direction: column;
        width: 100%;

        &:last-child {
            margin-left: 24px;
        }

        label {
            font-size: 14px;
            font-weight: 400;
            color: $gray900;
            margin-bottom: 8px;
        }

        @media only screen and (max-width: 550px) {
            margin-left: 0;
            max-width: none;

            select {
                max-width: none;
                height: 48px;
            }
        }
    }
}
</style>
