<script setup lang="ts">
import { computed } from 'vue'
import {
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
} from '~/shared'
import { usePriceStore } from '~/stores/price'
import { OfferType } from '~/types/components.interface'
import type { GetOffer, OfferTypeLabel } from '~/types/components.interface'
import { denomFromMicroDenom } from '~/utils/denom'

const props = defineProps<{ offer: GetOffer }>()
const { t } = useI18n()
const priceStore = usePriceStore()

const offerTypeLabels: OfferTypeLabel = { [OfferType.buy]: t('label.sell'), [OfferType.sell]: t('label.buy') }
const marginRate = computed(() => convertOfferRateToMarginRate(props.offer.rate))
const offerPrice = computed(() => {
  const usdRate = priceStore.getPrice(props.offer.fiat_currency)
  const fiatPrice = calculateFiatPriceByRate(usdRate, props.offer.rate)
  return `${props.offer.fiat_currency} ${formatAmount(fiatPrice, false)}`
})
</script>

<template>
  <div :key="`${offer.id}-collapsed`" class="offer collapsed">
    <div class="owner">
      <p class="wallet-addr">
        {{ formatAddress(offer.owner) }}
      </p>
      <p class="n-trades">
        0 trades
      </p>
    </div>

    <div class="info">
      <div class="wrap">
        <p class="label">
          Limits
        </p>
        <p class="limit">
          {{ formatAmount(offer.min_amount) }} -
          {{ formatAmount(offer.max_amount) }}
          {{ denomFromMicroDenom(offer.denom.native) }}
        </p>
      </div>

      <div class="divider" />
      <div class="wrap">
        <p class="label">
          Price
        </p>
        <p class="rate">
          {{ marginRate.marginOffset }}%
          {{ marginRate.margin }} market
        </p>
      </div>
    </div>

    <div class="price">
      <p class="value">
        {{ offerPrice }}
      </p>
      <button class="primary bg-gray300" type="button" @click="$emit('select')">
        {{ offerTypeLabels[offer.offer_type] }}
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.collapsed {
  .owner {
    display: flex;
    flex-direction: column;

    .wallet-addr {
      font-size: 18px;
      font-weight: 600;
      color: $base-text;
    }

    .n-trades {
      font-size: 14px;
      color: $gray700;
      margin-top: 4px;
    }

  }
}
</style>
