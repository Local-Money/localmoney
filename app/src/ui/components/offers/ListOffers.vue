<script setup lang="ts">
import type { GetOffer } from '~/types/components.interface'
import { FiatCurrency, OfferType } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { ExpandableItem } from '~/ui/components/util/ExpandableItem'
import { usePriceStore } from '~/stores/price'
import { defaultMicroDenomAvailable, denomsAvailable } from '~/utils/denom'
import { fiatsAvailable } from '~/utils/fiat'
import { checkValidOffer } from '~/utils/validations'

const client = useClientStore()
const priceStore = usePriceStore()
const offersResult = computed(() => client.offers)
const page = reactive({ offers: [] as ExpandableItem<GetOffer>[] })
client.$subscribe((mutation, state) => {
  if (state.offers.isSuccess()) {
    page.offers = state.offers.data
      .filter((offerResponse) => checkValidOffer(offerResponse.offer))
      .flatMap((offerResponse) => new ExpandableItem(offerResponse.offer))
  }
})

const selectedCrypto = ref<string>(defaultMicroDenomAvailable())
const fiatCurrency = ref<FiatCurrency>(FiatCurrency.ARS)
const offerType = ref<OfferType>(OfferType.sell)
const selectedOffer = ref<ExpandableItem<GetOffer> | null>(null)

function selectOffer(offer: ExpandableItem<GetOffer>) {
  if (selectedOffer.value !== null) {
    selectedOffer.value.isExpanded = false
  }
  offer.isExpanded = true
  selectedOffer.value = offer
}

function unselectOffer(offer: ExpandableItem<GetOffer>) {
  offer.isExpanded = false
}

async function fetchOffers() {
  await client.fetchOffers({
    fiatCurrency: fiatCurrency.value,
    offerType: offerType.value,
    denom: { native: selectedCrypto.value },
  })
}

onMounted(async () => {
  await priceStore.fetchPrices()
  await fetchOffers()
})

watch(fiatCurrency, async () => await fetchOffers())
watch(selectedCrypto, async () => await fetchOffers())
watch(offerType, async () => await fetchOffers())
</script>

<template>
  <section class="page">
    <p class="offers-section-title">Top offers from the community</p>
    <section class="offers-filter">
      <div class="buy-sell">
        <button class="buy" :class="{ focus: offerType === OfferType.sell }" @click="offerType = OfferType.sell">
          buy
        </button>
        <button class="sell" :class="{ focus: offerType === OfferType.buy }" @click="offerType = OfferType.buy">
          sell
        </button>
      </div>
      <div class="filter">
        <label for="crypto">Crypto</label>
        <CustomSelect v-model="selectedCrypto" :options="denomsAvailable" />
      </div>
      <div class="filter">
        <label for="currency">Currency (FIAT)</label>
        <CustomSelect v-model="fiatCurrency" :options="fiatsAvailable" />
      </div>
    </section>

    <section class="offers-list">
      <h3 v-if="offerType === OfferType.sell">Buy from these sellers</h3>
      <h3 v-if="offerType === OfferType.buy">Sell to these buyers</h3>
      <!-- Offers for -->
      <ListContentResult :result="offersResult" emptyStateMsg="There are no offers available yet">
        <ul>
          <li
            v-for="offer in page.offers"
            :key="offer.data.id"
            class="card"
            :class="offer.isExpanded ? 'card-active' : ''"
          >
            <!-- Collapsed Offer -->
            <CollapsedOffer v-if="!offer.isExpanded" :offer="offer.data" @select="selectOffer(offer)" />
            <!-- Expanded Offer Desktop -->
            <ExpandedOffer v-else :offer="offer.data" @cancel="unselectOffer(offer)" />
          </li>
        </ul>
      </ListContentResult>
    </section>
  </section>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

/* ----------- BUY SELL ROW */
.separator {
  margin: 0 auto 80px;
  display: flex;
  height: 1px;
  background-color: $border;

  @media only screen and (max-width: $mobile) {
    margin: 0 auto 32px;
  }
}

.offers-filter {
  display: flex;
  margin-bottom: 56px;

  @media only screen and (max-width: $mobile) {
    display: block;
    margin-bottom: 32px;
  }
}

.filter {
  display: inline-flex;
  flex-direction: column;
  width: 100%;
  max-width: 216px;
  margin-left: 24px;

  @media only screen and (max-width: $mobile) {
    max-width: none;
    margin-left: 0;
    margin-bottom: 16px;
  }

  label {
    font-size: 12px;
    color: $gray600;
    margin-bottom: 8px;
  }
}

.offers-section-title {
  font-size: 24px;
  margin-bottom: 40px;
  color: $gray900;
  font-weight: 600;

  @media only screen and (max-width: $mobile) {
    font-size: 18px;
    margin-bottom: 32px;
    text-align: center;
  }
}

/* ----------- OFFER LIST */
.offers-list {
  margin-top: 40px;
  margin-bottom: 56px;

  @media only screen and (max-width: $mobile) {
    margin-top: 24px;
  }

  h3 {
    color: $base-text;
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 32px;

    @media only screen and (max-width: $mobile) {
      margin: 16px 0 32px;
    }
  }

  li {
    list-style: none;
    margin-bottom: 24px;
  }

  .load-more {
    display: flex;
    justify-content: center;
    margin-top: 32px;

    button {
      padding: 0 48px;
    }
  }
}
</style>
