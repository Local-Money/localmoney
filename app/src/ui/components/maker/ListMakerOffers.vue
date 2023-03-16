<script setup lang="ts">
import type { OfferResponse } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { ExpandableItem } from '~/ui/components/util/ExpandableItem'
import { checkValidOffer } from '~/utils/validations'
import { OfferType } from '~/types/components.interface'

const client = useClientStore()
const route = useRoute()
const offersResult = computed(() => client.makerOffers)
const page = reactive({
  sellOffers: [] as ExpandableItem<OfferResponse>[],
  buyOffers: [] as ExpandableItem<OfferResponse>[],
})
client.$subscribe((mutation, state) => {
  if (state.makerOffers.isSuccess()) {
    state.makerOffers.data
      .filter((offerResponse) => checkValidOffer(offerResponse.offer, client.chainClient))
      .flatMap((offerResponse) => new ExpandableItem(offerResponse))
      .forEach((offerItem) => {
        switch (offerItem.data.offer.offer_type) {
          case OfferType.buy:
            page.buyOffers.push(offerItem)
            break
          case OfferType.sell:
            page.sellOffers.push(offerItem)
            break
        }
      })
  }
})

const selectedOfferItem = ref<ExpandableItem<OfferResponse> | null>(null)

function selectOffer(offerItem: ExpandableItem<OfferResponse>) {
  if (selectedOfferItem.value !== null) {
    selectedOfferItem.value.isExpanded = false
  }
  offerItem.isExpanded = true
  selectedOfferItem.value = offerItem
}

function unselectOffer(offerItem: ExpandableItem<OfferResponse>) {
  offerItem.isExpanded = false
}

async function updateFiatPrice() {
  // const denom: Denom = { native: selectedDenom.value }
  // await client.updateFiatPrice(selectedFiat.value, denom)
}

onBeforeMount(async () => {})

onMounted(async () => {
  const maker = (route.params.addr as string) ?? ''
  await client.fetchMakerOffers(maker)
  await updateFiatPrice()
})
</script>

<template>
  <section class="page">
    <section class="offers-list">
      <ListContentResult :result="offersResult" emptyStateMsg="There are no offers available yet">
        <p class="offers-section-title">Buy from this maker</p>
        <ul>
          <li v-for="offer in page.sellOffers" :key="offer.data.id" :class="offer.isExpanded ? 'card-active' : ''">
            <!-- Collapsed Offer -->
            <CollapsedOffer v-if="!offer.isExpanded" :offerResponse="offer.data" @select="selectOffer(offer)" />
            <!-- Expanded Offer Desktop -->
            <ExpandedOffer v-else :offerResponse="offer.data" @cancel="unselectOffer(offer)" />
          </li>
        </ul>
        <p class="offers-section-title">Sell from this maker</p>
        <ul>
          <li v-for="offer in page.buyOffers" :key="offer.data.id" :class="offer.isExpanded ? 'card-active' : ''">
            <!-- Collapsed Offer -->
            <CollapsedOffer v-if="!offer.isExpanded" :offerResponse="offer.data" @select="selectOffer(offer)" />
            <!-- Expanded Offer Desktop -->
            <ExpandedOffer v-else :offerResponse="offer.data" @cancel="unselectOffer(offer)" />
          </li>
        </ul>
      </ListContentResult>
    </section>
  </section>
</template>

<style lang="scss" scoped>
@import '../../style/tokens';

section {
  margin-top: 0;
}

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
