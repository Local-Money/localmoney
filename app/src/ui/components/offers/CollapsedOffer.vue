<script setup lang="ts">
import { computed } from 'vue'
import {
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  formatTradesCountInfo,
} from '~/shared'
import { usePriceStore } from '~/stores/price'
import { OfferType } from '~/types/components.interface'
import type { GetOffer, OfferResponse, OfferTypeLabel } from '~/types/components.interface'
import { microDenomToDenom } from '~/utils/denom'

const props = defineProps<{ offerResponse: OfferResponse }>()
const emit = defineEmits<{ (e: 'select'): void }>()
const { t } = useI18n()
const priceStore = usePriceStore()

const offerTypeLabels: OfferTypeLabel = { [OfferType.buy]: t('label.sell'), [OfferType.sell]: t('label.buy') }
const marginRate = computed(() => convertOfferRateToMarginRate(props.offerResponse.offer.rate))
const offerPrice = computed(() => {
  const usdRate = priceStore.getPrice(props.offerResponse.offer.fiat_currency)
  const fiatPrice = calculateFiatPriceByRate(usdRate, props.offerResponse.offer.rate)
  return `${props.offerResponse.offer.fiat_currency} ${formatAmount(fiatPrice, false)}`
})
</script>

<template>
  <div :key="`${offerResponse.offer.id}-collapsed`" class="offer collapsed">
    <div class="owner">
      <p class="wallet-addr">
        {{ formatAddress(offerResponse.offer.owner) }}
      </p>
      <p class="n-trades">{{ formatTradesCountInfo(offerResponse.profile.released_trades_count) }}</p>
    </div>

    <div class="info">
      <div class="wrap">
        <p class="label">Limits</p>
        <p class="limit">
          {{ formatAmount(offerResponse.offer.min_amount) }} -
          {{ formatAmount(offerResponse.offer.max_amount) }}
          {{ microDenomToDenom(offerResponse.offer.denom.native) }}
        </p>
      </div>

      <div class="divider" />
      <div class="wrap">
        <p class="label">Price</p>
        <p class="rate">{{ marginRate.marginOffset }}% {{ marginRate.margin }} market</p>
      </div>
    </div>

    <div class="price">
      <p class="value">
        {{ offerPrice }}
      </p>
      <button class="primary bg-gray300" type="button" @click="emit('select')">
        {{ offerTypeLabels[offerResponse.offer.offer_type] }}
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.collapsed {
  .owner {
    display: flex;
    flex-direction: column;

    @media only screen and (max-width: $mobile) {
      flex-direction: row;
      justify-content: space-between;
    }

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
