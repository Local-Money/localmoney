<script setup lang="ts">
import { ref } from 'vue'
import CurrencyInput from '../CurrencyInput.vue'
import {
  calculateFiatPriceByRate,
  convertMarginRateToOfferRate,
  convertOfferRateToMarginRate,
  formatAmount,
} from '~/shared'
import { microDenomToDisplay } from '~/utils/denom'
import type { GetOffer } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'

const props = defineProps<{ offer: GetOffer }>()
const emit = defineEmits<{ (e: 'cancel'): void }>()
const client = useClientStore()
const updatedOffer = ref<GetOffer>({
  ...props.offer,
  min_amount: `${formatAmount(props.offer.min_amount, true, 6)}`,
  max_amount: `${formatAmount(props.offer.max_amount, true, 6)}`,
})

const marginRate = computed(() => convertOfferRateToMarginRate(props.offer.rate))
const margin = ref(marginRate.value.margin)
const marginOffset = ref(marginRate.value.marginOffset)
const minAmount = ref(Number(props.offer.min_amount) / 1000000)
const maxAmount = ref(Number(props.offer.max_amount) / 1000000)
const description = ref(updatedOffer.value.description)
const rate = ref(props.offer.rate)
const valid = ref(true)

// watch min amount and max amount and update their respective values in the props.offer
watch(minAmount, (val) => {
  updatedOffer.value.min_amount = `${val}`
  valid.value = Number(updatedOffer.value.min_amount) < Number(updatedOffer.value.max_amount)
})
watch(maxAmount, (val) => {
  updatedOffer.value.max_amount = `${val}`
  valid.value = Number(updatedOffer.value.min_amount) < Number(updatedOffer.value.max_amount)
})

const fiatPriceByRate = computed(() => {
  const denomFiatPrice = client.getFiatPrice(props.offer.fiat_currency, props.offer.denom)
  return calculateFiatPriceByRate(denomFiatPrice, rate.value)
})
const offerPrice = computed(() => `${props.offer.fiat_currency} ${formatAmount(fiatPriceByRate.value, false)}`)

function update() {
  const offer = updatedOffer.value
  client.updateOffer({
    id: offer.id,
    state: offer.state,
    rate: `${convertMarginRateToOfferRate(marginRate.value.margin, marginRate.value.marginOffset)}`,
    min_amount: `${(Number(offer.min_amount) * 1000000).toFixed(0)}`,
    max_amount: `${(Number(offer.max_amount) * 1000000).toFixed(0)}`,
    description: description.value,
  })
}
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
      <div class="div outer-wrap">
        <div class="inner-wrap">
          <div class="input-wrap">
            <p class="label">Min amount</p>
            <CurrencyInput
              v-model="minAmount"
              placeholder="Offer min amount"
              :decimals="6"
              :isCrypto="true"
              :prefix="microDenomToDisplay(offer.denom.native, client.chainClient)"
            />
          </div>

          <div class="input-wrap">
            <p class="label">Max amount</p>
            <CurrencyInput
              v-model="maxAmount"
              placeholder="Offer max amount"
              :decimals="6"
              :isCrypto="true"
              :prefix="microDenomToDisplay(offer.denom.native, client.chainClient)"
            />
          </div>
        </div>
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
      </div>
      <div class="bottom-wrap">
        <div class="description">
          <div class="wrap">
            <div class="wrap-label">
              <label>Edit offer description</label>
              <IconTooltip content="Here you can write the payment options you will be accepting for this offer." />
            </div>
            <textarea
              v-model="description"
              maxlength="90"
              minlength="3"
              placeholder="Bank transfer, Paypal, Cash..."
            ></textarea>
          </div>
        </div>
        <div class="wrap-btns">
          <button class="secondary" @click="emit('cancel')">cancel</button>
          <button class="primary bg-gray300" :disabled="!valid" @click="update()">update</button>
        </div>
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
    flex-direction: column;
    align-items: center;
    gap: 24px;

    .outer-wrap {
      width: 100%;
      display: flex;
      gap: 32px;

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
          color: $gray900;
          margin-bottom: 8px;
        }

        input {
          color: $base-text;
          background-color: $background;
          text-align: right;
        }
      }
    }
    .bottom-wrap {
      width: 100%;
      display: flex;
      align-items: flex-end;
      justify-content: space-between;

      @include responsive(mobile) {
        flex-direction: column;
        align-items: center;
        gap: 24px;
      }

      .description {
        flex: 2;
        @include responsive(mobile) {
          width: 100%;
        }

        .wrap-label {
          display: flex;
          gap: 8px;
        }

        label {
          font-size: 14px;
          font-weight: 400;
          color: $gray900;
          margin-bottom: 8px;

          @media only screen and (max-width: $mobile) {
            font-size: 12px;
          }
        }
        textarea {
          background-color: $background;
        }
      }
      .wrap-btns {
        display: flex;
        justify-content: flex-end;
        gap: 24px;
        flex: 1;
        margin-bottom: 16px;
      }
    }
  }
}
</style>
