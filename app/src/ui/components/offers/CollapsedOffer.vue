<script setup lang="ts">
import { computed } from 'vue'
import {
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  formatTradesCountInfo,
} from '~/shared'
import { OfferType } from '~/types/components.interface'
import type { OfferResponse, OfferTypeLabel } from '~/types/components.interface'
import { microDenomToDenom } from '~/utils/denom'
import { useClientStore } from '~/stores/client'

const props = defineProps<{ offerResponse: OfferResponse }>()
const emit = defineEmits<{ (e: 'select'): void }>()
const { t } = useI18n()
const client = useClientStore()

const offerTypeLabels: OfferTypeLabel = { [OfferType.buy]: t('label.sell'), [OfferType.sell]: t('label.buy') }
const marginRate = computed(() => convertOfferRateToMarginRate(props.offerResponse.offer.rate))
const offerPrice = computed(() => {
  const offer = props.offerResponse.offer
  const denomFiatPrice = client.fiatPrices.get(offer.fiat_currency)?.get(offer.denom.native)
  const fiatPrice = calculateFiatPriceByRate(denomFiatPrice, props.offerResponse.offer.rate) / 100
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

    <div class="inner-wrap">
      <div class="info">
        <div class="wrap">
          <p class="label">Trade limit</p>
          <p class="limit">
            {{ formatAmount(offerResponse.offer.min_amount) }} -
            {{ formatAmount(offerResponse.offer.max_amount) }}
            {{ microDenomToDenom(offerResponse.offer.denom.native) }}
          </p>
        </div>
        <div class="divider"></div>
        <div class="description">
          <p class="content">Lemon Cash, Bank Transfer (Argentina), Mercado Pago, RebaBanco, Brubank</p>
        </div>
      </div>

      <div class="price">
        <div class="wrap">
          <p class="value">
            {{ offerPrice }}
          </p>
          <p class="margin">{{ marginRate.marginOffset }}% {{ marginRate.margin }} market</p>
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
  .owner {
    width: 20%;
    display: flex;
    flex-direction: column;

    @include responsive(mobile) {
      width: 100%;
      flex-direction: row;
      justify-content: space-between;
      margin-bottom: 24px;
    }

    .wallet-addr {
      font-size: 16px;
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
