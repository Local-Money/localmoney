<script setup lang="ts">
import type { Addr, OfferResponse } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { ExpandableItem } from '~/ui/components/util/ExpandableItem'
import { checkValidOffer } from '~/utils/validations'
import { OfferType } from '~/types/components.interface'

const props = defineProps<{ maker: Addr }>()
const client = useClientStore()
const offersResult = computed(() => client.makerOffers)
const page = reactive({
  sellOffers: [] as ExpandableItem<OfferResponse>[],
  buyOffers: [] as ExpandableItem<OfferResponse>[],
})
client.$subscribe((mutation, state) => {
  if (state.makerOffers.isSuccess()) {
    const sellOffers = [] as ExpandableItem<OfferResponse>[]
    const buyOffers = [] as ExpandableItem<OfferResponse>[]
    state.makerOffers.data
      .filter((offerResponse) => checkValidOffer(offerResponse.offer, client.chainClient))
      .flatMap((offerResponse) => new ExpandableItem(offerResponse))
      .forEach((offerItem) => {
        switch (offerItem.data.offer.offer_type) {
          case OfferType.buy:
            buyOffers.push(offerItem)
            break
          case OfferType.sell:
            sellOffers.push(offerItem)
            break
        }
      })
    page.buyOffers = buyOffers
    page.sellOffers = sellOffers
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
  await client.fetchMakerOffers(props.maker)
  await updateFiatPrice()
})
</script>

<template>
  <section class="offers-list">
    <ListContentResult :result="offersResult" emptyStateMsg="This user has no active offers">
      <slot v-if="page.sellOffers.length > 0">
        <p class="offers-section-title">Buy from this user</p>
        <ul>
          <li v-for="offer in page.sellOffers" :key="offer.data.id" :class="offer.isExpanded ? 'card-active' : ''">
            <!-- Collapsed Offer -->
            <CollapsedMakerOffer v-if="!offer.isExpanded" :offerResponse="offer.data" @select="selectOffer(offer)" />
            <!-- Expanded Offer Desktop -->
            <ExpandedMakerOffer v-else :offerResponse="offer.data" @cancel="unselectOffer(offer)" />
          </li>
        </ul>
      </slot>
      <slot v-if="page.buyOffers.length > 0">
        <p class="offers-section-title">Sell to this user</p>
        <ul>
          <li v-for="offer in page.buyOffers" :key="offer.data.id" :class="offer.isExpanded ? 'card-active' : ''">
            <!-- Collapsed Offer -->
            <CollapsedMakerOffer v-if="!offer.isExpanded" :offerResponse="offer.data" @select="selectOffer(offer)" />
            <!-- Expanded Offer Desktop -->
            <ExpandedMakerOffer v-else :offerResponse="offer.data" @cancel="unselectOffer(offer)" />
          </li>
        </ul>
      </slot>
    </ListContentResult>
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

.offers-section-title {
  font-size: 18px;
  margin-bottom: 40px;
  color: $gray900;
  font-weight: $semi-bold;

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

  ul {
    margin-bottom: 64px;
    &:last-child {
      margin-bottom: 0;
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
