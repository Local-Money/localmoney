<script setup lang="ts">
import CurrencyInput from '../CurrencyInput.vue'
import {
  calculateFiatPriceByRate,
  convertMarginRateToOfferRate,
  convertOfferRateToMarginRate,
  formatAmount,
} from '~/shared'
import type { GetOffer } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'

const props = defineProps<{ offer: GetOffer }>()
const emit = defineEmits<{ (e: 'cancel'): void }>()
const client = useClientStore()
const updatedOffer = ref<GetOffer>({
  ...props.offer,
  min_amount: `${formatAmount(props.offer.min_amount)}`,
  max_amount: `${formatAmount(props.offer.max_amount)}`,
})
const marginRate = computed(() => convertOfferRateToMarginRate(props.offer.rate))
const margin = ref(marginRate.value.margin)
const marginOffset = ref(marginRate.value.marginOffset)
const rate = ref(0)
const fiatPriceByRate = computed(() => {
  const denomFiatPrice = client.getFiatPrice(props.offer.fiat_currency, props.offer.denom)
  return calculateFiatPriceByRate(denomFiatPrice, rate.value)
})
const offerPrice = computed(() => `${props.offer.fiat_currency} ${formatAmount(fiatPriceByRate.value, false)}`)
const valid = computed(() => updatedOffer.value.max_amount > updatedOffer.value.min_amount)

function calculateMarginRate() {
  rate.value = convertMarginRateToOfferRate(marginRate.value.margin, marginRate.value.marginOffset)
}

function update() {
  const offer = updatedOffer.value
  client.updateOffer({
    id: offer.id,
    state: offer.state,
    rate: `${rate.value}`,
    min_amount: `${formatAmount(offer.min_amount, false) * 1000000}`,
    max_amount: `${formatAmount(offer.max_amount, false) * 1000000}`,
  })
}

watch(margin, (val) => {
  marginRate.value.margin = val
  calculateMarginRate()
})

watch(marginOffset, () => {
  calculateMarginRate()
})
</script>

<template>
  <div :key="`${offer.id}-expanded`" ref="expandedCard" class="offer expanded">
    <div class="offer-type">
      <div class="wrap-status">
        <select v-model="updatedOffer.state" class="bg-gray100">
          <option value="active">Active</option>
          <option value="paused">Pause</option>
          <option value="archive">Archive</option>
        </select>
      </div>
      <div class="inner-wrap">
        <p class="type">{{ updatedOffer.offer_type }}ing</p>
        <p class="value">
          {{ offerPrice }}
        </p>
      </div>
    </div>

    <div class="horizontal-separator" />

    <div class="wrap-edit">
      <div class="inner-wrap">
        <div class="input-wrap">
          <p class="label">Margin</p>
          <select v-model="margin" class="bg-gray100">
            <option value="above">Above</option>
            <option value="below">Below</option>
          </select>
        </div>

        <div class="input-wrap">
          <p class="label">Margin offset</p>
          <input
            v-model="marginOffset"
            v-maska="['##%', '#%']"
            placeholder="0%"
            @maska="marginRate.marginOffset = $event.target.dataset.maskRawValue"
          />
        </div>
      </div>

      <div class="inner-wrap">
        <div class="input-wrap">
          <p class="label">Min amount:</p>
          <CurrencyInput
            v-model="updatedOffer.min_amount"
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

        <div class="input-wrap">
          <p class="label">Max amount:</p>
          <CurrencyInput
            v-model="updatedOffer.max_amount"
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
      <div class="wrap-btns">
        <button class="secondary" @click="emit('cancel')">cancel</button>
        <button class="primary" :disabled="!valid" @click="update()">update</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.expanded {
  display: flex;
  flex-direction: column;

  .offer-type {
    display: flex;
    align-items: center;
    gap: 32px;

    @media only screen and (max-width: $mobile) {
      flex-direction: column-reverse;
      align-items: flex-start;
      gap: 24px;
    }

    .wrap-status {
      display: flex;
      align-items: center;
      min-width: 150px;

      @media only screen and (max-width: $mobile) {
        width: 100%;
      }

      select {
        color: $primary;
      }
    }

    .inner-wrap {
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 32px;

      .type {
        font-size: 18px;
        font-weight: $semi-bold;
        color: $base-text;
        text-transform: capitalize;
      }

      .value {
        font-size: 20px;
        color: $base-text;
        font-weight: $bold;
        margin-right: 24px;

        @media only screen and (max-width: $mobile) {
          margin-right: 0px;
        }
      }
    }
  }

  .horizontal-separator {
    width: 100%;
    height: 1px;
    background-color: $border;
    margin: 32px 0 0px;
  }

  .wrap-edit {
    display: flex;
    align-items: center;
    gap: 32px;

    @media only screen and (max-width: $mobile) {
      flex-direction: column;
      gap: 16px;
    }

    .inner-wrap {
      width: 100%;
      margin-top: 16px;
      display: flex;
      gap: 16px;
      padding: 8px 0px;

      @media only screen and (max-width: $mobile) {
        flex-direction: column;
      }

      .input-wrap {
        display: flex;
        flex-direction: column;
        width: 100%;
      }

      .label {
        font-size: 14px;
        color: $gray600;
        margin-bottom: 8px;
      }

      input {
        color: $base-text;
        background-color: $background;
        text-align: right;
      }
    }
  }

  .wrap-btns {
    display: flex;
    justify-content: flex-end;
    gap: 24px;
    margin-top: 32px;
  }
}
</style>
