<script setup lang="ts">
import ExpandedMyOffer from '~/ui/components/myOffers/ExpandedMyOffer.vue'
import CollapsedMyOffer from '~/ui/components/myOffers/CollapsedMyOffer.vue'
import ArchivedOfferItem from '~/ui/components/myOffers/ArchivedOfferItem.vue'
import { useClientStore } from '~/stores/client'
import { ExpandableItem } from '~/ui/components/util/ExpandableItem'
import type { GetOffer } from '~/types/components.interface'
import { OfferState } from '~/types/components.interface'
import { ListResult } from '~/stores/ListResult'
import { checkMicroDenomAvailable } from '~/utils/denom'

const client = useClientStore()
const myOffersResult = computed<ListResult<GetOffer>>(() => client.myOffers)
const page = reactive({
  myOffers: <ExpandableItem<GetOffer>[]>[],
  archiveOffers: <GetOffer[]>[],
})
client.$subscribe((mutation, state) => {
  if (state.myOffers.isSuccess()) {
    page.myOffers = state.myOffers.data
        .filter(offer => checkMicroDenomAvailable(offer.denom.native) && offer.state !== OfferState.archived)
        .flatMap(offer => new ExpandableItem(offer))

    page.archiveOffers = state.myOffers.data
        .filter(offer => checkMicroDenomAvailable(offer.denom.native) && offer.state === OfferState.archived)
  }
})
const expandedMyOffer = ref()

function hasOffers() {
  return page.myOffers.length > 0
}

function hasArchivedOffers() {
  return page.archiveOffers.length > 0
}

function expandOfferItem(offer: ExpandableItem<GetOffer>) {
  if (expandedMyOffer.value !== offer) {
    if (expandedMyOffer.value != null)
      expandedMyOffer.value.isExpanded = false

    offer.isExpanded = true
    expandedMyOffer.value = offer
  }
}

function collapseOfferItem(offer: ExpandableItem<GetOffer>) {
  offer.isExpanded = false
  expandedMyOffer.value = null
}

async function loadMore() {
  await client.fetchMyOffers()
}

onMounted(async () => {
  await client.fetchMyOffers()
})
</script>

<template>
  <section>
    <ListContentResult
      :result="myOffersResult"
      :emptyStateMsg="'There is no offers available yet'"
    >
      <!-- My Offers section -->
      <section v-if="hasOffers()" class="offers-list">
        <!-- Offers for -->
        <ul>
          <li
            v-for="offer in page.myOffers"
            :key="offer.data.id"
            class="card"
            :class="offer.isExpanded ? 'card-active' : ''"
          >
            <!-- Collapsed Offer -->
            <CollapsedMyOffer
              v-if="!offer.isExpanded"
              :offer="offer.data"
              @select="expandOfferItem(offer)"
            />
            <!-- Expanded Offer Desktop -->
            <ExpandedMyOffer
              v-else
              :offer="offer.data"
              @cancel="collapseOfferItem(offer)"
            />
          </li>
        </ul>
      </section>
      <section v-else-if="!hasOffers()" class="card">
        <p>
          Nothing here yet
        </p>
      </section>

      <!-- End My Offers section -->
      <!-- Archived offers table -->
      <h3 v-if="hasArchivedOffers()">
        Archived Offers
      </h3>
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
          v-for="offer in page.archiveOffers"
          :key="offer.id"
          :offer="offer"
        />
      </section>
      <!-- End Archived offers table -->
    </ListContentResult>
  </section>

  <!-- Expanded Offer Mobile -->
  <div v-if="false" class="expanded-mobile">
    <div class="owner">
      <p class="wallet">
        terra12242343
      </p>
      <p class="n-trades">
        352 trades
      </p>
    </div>

    <div class="payment-info">
      <p class="note">
        Nubank, Itaú, C6, Mercado Pago, Inter, Caixa, Bradesco
      </p>
    </div>

    <form action="">
      <div class="input">
        <label for="buy">I want to buy</label>
        <input ref="buyAmountInput" type="text" placeholder="100.00">
      </div>
      <div class="input">
        <label for="sell">I will receive</label>
        <input type="text" placeholder="100.00">
        <p>Min - 1 | Max - 50</p>
      </div>
    </form>

    <div class="receipt">
      <div class="price">
        <div class="wrap-price">
          <p class="label">
            Price
          </p>
          <p class="ticker">
            Will refresh in 47s
          </p>
        </div>
        <div class="wrap">
          <p class="margin">
            4% above market
          </p>
          <p class="value">
            COL$ 3.695,59
          </p>
        </div>
      </div>

      <div class="sumary">
        <p class="label">
          Transaction sumary
        </p>
        <div class="wrap">
          <div class="item">
            <p class="info">
              Trading Fee
            </p>
            <p>COL$ 3.695,59</p>
          </div>
          <div class="item">
            <p class="info">
              You will get
            </p>
            <p class="price-get">
              100.00
            </p>
          </div>
          <div class="item">
            <p class="info">
              You will pay
            </p>
            <p class="price-pay">
              COP$ 348.892,53
            </p>
          </div>
        </div>
      </div>
    </div>

    <div class="wrap-btns">
      <button class="secondary">
        cancel
      </button>
      <button class="primary">
        open transaction
      </button>
    </div>
  </div>
  <!-- Expanded Offer Mobile -->
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

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
    font-family: "Poppins", sans-serif;
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

/* -------------- Expanded Mobile */
.expanded-mobile {
  position: absolute;
  width: 100%;
  height: 100vh;
  display: grid;
  grid-template-columns: 1fr;
  background-color: $white;

  .owner {
    grid-column: 1/1;
    grid-row: 1;
    padding: 16px 24px 0;

    .wallet {
      font-size: 18px;
      font-weight: 600;
      color: $base-text;
    }

    .n-trades {
      font-size: 14px;
      color: $gray600;
    }
  }

  .payment-info {
    grid-column: 1/1;
    grid-row: 2;
    padding: 0px 24px;

    .note {
      font-size: 14px;
      color: $gray600;
    }
  }

  form {
    grid-column: 1/1;
    grid-row: 3;
    margin-top: 16px;
    padding: 0px 24px;

    .input:first-child {
      margin-bottom: 24px;
    }

    label {
      font-size: 14px;
      color: $gray600;
      display: block;
    }

    input {
      width: 100%;
      font-family: "Poppins", sans-serif;
      font-size: 16px;
      font-weight: 800;
      line-height: 24px;
      color: $base-text;
      padding: 10px 16px;
      border: 1px solid $border;
      border-radius: 8px;
      margin-top: 8px;
      text-align: right;
    }

    p {
      font-size: 12px;
      color: $gray600;
      text-align: right;
      margin-top: 8px;
    }
  }

  .receipt {
    grid-column: 1/1;
    grid-row: 4;
    margin-top: 16px;
    background-color: $background;
    border-top: 1px solid $border;
    border-bottom: 1px solid $border;
    padding: 16px 24px;

    .price {
      margin-bottom: 24px;

      .wrap-price {
        display: flex;
        justify-content: space-between;
      }

      .label {
        display: inline-block;
        font-size: 14px;
        color: $gray600;
      }

      .ticker {
        display: inline-block;
        font-size: 12px;
        color: $primary;
      }

      .wrap {
        width: 100%;
        display: inline-flex;
        justify-content: space-between;
        border: 1px solid $border;
        background-color: $white;
        border-radius: 8px;
        padding: 10px 24px;
        margin-top: 8px;
        align-items: center;
        gap: 16px;

        .margin {
          font-size: 14px;
          max-width: 100px;
          color: $gray600;
        }

        .value {
          font-size: 16px;
          font-weight: 600;
          color: $base-text;
        }
      }
    }

    .sumary {
      margin-bottom: 24px;

      .label {
        font-size: 14px;
        color: $gray600;
      }

      .wrap {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        border: 1px solid $border;
        background-color: $white;
        border-radius: 8px;
        padding: 16px 24px;
        margin-top: 8px;

        gap: 8px;

        .item {
          display: inline-flex;
          justify-content: space-between;

          .price-get {
            font-weight: 800;
          }

          .price-pay {
            font-weight: 800;
            color: $primary;
          }
        }
      }
    }
  }
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
