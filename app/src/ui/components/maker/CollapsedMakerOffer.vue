<script setup lang="ts">
import { computed } from 'vue'
import {
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  formatTradesCountInfo,
} from '~/shared'
import type { OfferResponse, OfferTypeLabel } from '~/types/components.interface'
import { OfferType } from '~/types/components.interface'
import { denomToValue, microDenomToDisplay, microDenomToIcon } from '~/utils/denom'
import { useClientStore } from '~/stores/client'

const props = defineProps<{ offerResponse: OfferResponse }>()
const emit = defineEmits<{ (e: 'select'): void }>()
const { t } = useI18n()
const client = useClientStore()

const offerTypeLabels: OfferTypeLabel = { [OfferType.buy]: t('label.sell'), [OfferType.sell]: t('label.buy') }
const marginRate = computed(() => convertOfferRateToMarginRate(props.offerResponse.offer.rate))
const offerPrice = computed(() => {
  const offer = props.offerResponse.offer
  const denomFiatPrice = client.fiatPrices.get(offer.fiat_currency)?.get(denomToValue(offer.denom))
  const fiatPrice = calculateFiatPriceByRate(denomFiatPrice, props.offerResponse.offer.rate) / 100
  return `${props.offerResponse.offer.fiat_currency} ${formatAmount(fiatPrice, false)}`
})
const tradeCountIcon = computed(() => props.offerResponse.profile.released_trades_count > 0)
</script>

<template>
  <div :key="`${offerResponse.offer.id}-collapsed`" class="offer collapsed card" @click="emit('select')">
    <div class="offer-token">
      <img :src="microDenomToIcon(offerResponse.offer.denom.native, client.chainClient)" />
      <p>{{ microDenomToDisplay(offerResponse.offer.denom.native, client.chainClient) }}</p>
    </div>

    <div class="inner-wrap">
      <div class="info">
        <div class="wrap">
          <p class="label">Trade limit</p>
          <p class="limit">
            {{ formatAmount(offerResponse.offer.min_amount, true, 6) }} -
            {{ formatAmount(offerResponse.offer.max_amount, true, 6) }}
            {{ microDenomToDisplay(offerResponse.offer.denom.native, client.chainClient) }}
          </p>
        </div>
        <div class="divider"></div>
        <div class="description">
          <p class="content">{{ offerResponse.offer.description ?? 'No Description' }}</p>
        </div>
      </div>

      <div class="price">
        <div class="wrap">
          <p class="value">
            {{ offerPrice }}
          </p>
        </div>
        <button class="primary bg-gray300" type="button" @click="emit('select')">
          {{ offerTypeLabels[offerResponse.offer.offer_type] }}
        </button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.collapsed {
  cursor: pointer;
  .offer-token {
    width: 20%;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 16px;

    img {
      width: 32px;
      height: 32px;
    }

    p {
      font-size: 20px;
      font-weight: $semi-bold;
    }

    @include responsive(mobile) {
      width: 100%;
      flex-direction: row;
      justify-content: space-between;
      margin-bottom: 24px;
    }
  }
}
</style>
