<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useClientStore } from '~/stores/client'
import type { ListResult } from '~/stores/ListResult'
import type { OfferResponse } from '~/types/components.interface'
import { OfferState } from '~/types/components.interface'
import ArchivedOfferItem from '~/ui/components/myOffers/ArchivedOfferItem.vue'
import CollapsedMyOffer from '~/ui/components/myOffers/CollapsedMyOffer.vue'
import ExpandedMyOffer from '~/ui/components/myOffers/ExpandedMyOffer.vue'
import { ExpandableItem } from '~/ui/components/util/ExpandableItem'
import { checkValidOffer } from '~/utils/validations'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
const myOffersResult = computed<ListResult<OfferResponse>>(() => client.myOffers)
const paginationLastItem = ref<number>(0)
const page = reactive({
  myOffers: [] as ExpandableItem<OfferResponse>[],
  archiveOffers: [] as OfferResponse[],
})
client.$subscribe((mutation, state) => {
  if (state.myOffers.isSuccess()) {
    page.myOffers = state.myOffers.data
      .filter(
        (offerResponse) =>
          checkValidOffer(offerResponse.offer, client.chainClient) && offerResponse.offer.state !== OfferState.archived
      )
      .flatMap((offerResponse) => new ExpandableItem(offerResponse))

    page.archiveOffers = state.myOffers.data.filter(
      (offerResponse) =>
        checkValidOffer(offerResponse.offer, client.chainClient) && offerResponse.offer.state === OfferState.archived
    )
  }
})
const expandedMyOffer = ref()

function hasOffers() {
  return page.myOffers.length > 0
}

function hasArchivedOffers() {
  return page.archiveOffers.length > 0
}

function expandOfferItem(offerItem: ExpandableItem<OfferResponse>) {
  if (expandedMyOffer.value !== offerItem) {
    if (expandedMyOffer.value != null) {
      expandedMyOffer.value.isExpanded = false
    }

    offerItem.isExpanded = true
    expandedMyOffer.value = offerItem
  }
}

function collapseOfferItem(offerItem: ExpandableItem<OfferResponse>) {
  offerItem.isExpanded = false
  expandedMyOffer.value = null
}

async function loadMore() {
  const lastIndex = myOffersResult.value.data.length
  paginationLastItem.value = lastIndex > 0 ? myOffersResult.value.data[lastIndex - 1].offer.id : 0
  await client.fetchMoreMyOffers(paginationLastItem.value)
}

onMounted(async () => {
  await client.fetchMyOffers()
})

watch(userWallet, async () => {
  await client.fetchMyOffers()
})
</script>

<template>
  <section>
    <ListContentResult
      :result="myOffersResult"
      emptyStateMsg="There are no offers available yet"
      @loadMore="loadMore()"
    >
      <!-- My Offers section -->
      <section v-if="hasOffers()" class="offers-list">
        <!-- Offers for -->
        <ul>
          <li
            v-for="offerItem in page.myOffers"
            :key="offerItem.data.id"
            class="card"
            :class="offerItem.isExpanded ? 'card-active' : ''"
          >
            <!-- Collapsed Offer -->
            <CollapsedMyOffer
              v-if="!offerItem.isExpanded"
              :offer="offerItem.data.offer"
              @select="expandOfferItem(offerItem)"
            />
            <!-- Expanded Offer Desktop -->
            <ExpandedMyOffer v-else :offer="offerItem.data.offer" @cancel="collapseOfferItem(offerItem.data.offer)" />
          </li>
        </ul>
      </section>
      <section v-else-if="!hasOffers()" class="card">
        <p>Nothing here yet</p>
      </section>
      <!-- End My Offers section -->

      <!-- Archived offers table -->
      <h3 v-if="hasArchivedOffers()">Archived Offers</h3>
      <section v-if="hasArchivedOffers()" class="archived-offers-table card">
        <div class="table-header">
          <div class="col-1">
            <p>Date</p>
          </div>
          <div class="col-2">
            <p>Type</p>
          </div>
          <div class="col-3">
            <p>Fiat</p>
          </div>
          <div class="col-4">
            <p>Limits</p>
          </div>
          <div class="col-5">
            <p>Price</p>
          </div>
          <div class="col-6" />
        </div>
        <ArchivedOfferItem
          v-for="offerResult in page.archiveOffers"
          :key="offerResult.offer.id"
          :offer="offerResult.offer"
        />
      </section>
      <!-- End Archived offers table -->
    </ListContentResult>
  </section>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

/* ----------- BUY SELL ROW */
.separator {
  margin: 0 auto 42px;
  display: flex;
  height: 1px;
  background-color: $border;
}

.offers-filter {
  display: flex;

  @media only screen and (max-width: 550px) {
    display: block;
  }
}

.filter {
  display: inline-flex;
  flex-direction: column;
  width: 100%;
  max-width: 200px;
  margin-left: 24px;

  label {
    font-size: 12px;
    color: $gray600;
    margin-bottom: 8px;
  }

  select {
    width: 100%;
    max-width: 200px;
    background-color: $surface;
    border-radius: 8px;
    border: 1px solid $border;
    font-family: 'Poppins', sans-serif;
    font-size: 14px;
    font-weight: 600;
    color: $base-text;
    padding: 10px 16px;
    appearance: none;
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

/* ----------- OFFER LIST */
.offers-list {
  h3 {
    color: $base-text;
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 32px;
  }

  li {
    list-style: none;
    margin-bottom: 24px;
  }
}

.load-more {
  display: flex;
  justify-content: center;
  margin-top: 32px;

  button {
    padding: 0 48px;
  }
}

.loading-content {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

.wrap-btns {
  grid-template-columns: 1/1;
  grid-row: 5;
  display: flex;
  justify-content: space-around;
  padding: 24px 0px;

  .primary {
    background-color: #ef6100;
    color: $white;
    border: none;
    font-family: inherit;
    font-weight: 700;
    font-size: 16px;
    text-transform: lowercase;
    padding: 8px 24px;
  }

  .secondary {
    color: $primary;
    border: none;
    font-family: inherit;
    font-weight: 700;
    font-size: 16px;
    text-transform: lowercase;
    padding: 8px 24px;
  }
}

/* ----------- ARCHIVED OFFERS TABLE */
.archived-offers-table {
  .table-header {
    display: flex;
    flex-direction: row;
    border-bottom: 1px solid $border;
    padding: 16px;
    margin-bottom: 16px;

    p {
      font-size: 14px;
      font-weight: $semi-bold;
      color: $gray700;
    }
  }
}

.col-1,
:deep(.col-1) {
  width: 12.5%;
}

.col-2,
:deep(.col-2) {
  width: 12.5%;
}

.col-3,
:deep(.col-3) {
  width: 12.5%;
}

.col-4,
:deep(.col-4) {
  width: 33.5%;
}

.col-5,
:deep(.col-5) {
  width: 20%;
}

.col-6,
:deep(.col-6) {
  width: 10%;
}

/* ----------- END ARCHIVED OFFERS TABLE */
</style>
