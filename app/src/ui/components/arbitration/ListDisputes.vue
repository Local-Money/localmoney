<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useClientStore } from '~/stores/client'
import { microDenomToDisplay } from '~/utils/denom'
import { formatTradeState } from '~/shared'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
const openDisputeResult = computed(() => client.openDisputes)
const openDisputes = computed(() => {
  return openDisputeResult.value.isSuccess() ? openDisputeResult.value.data : []
})
const closedDisputesResult = computed(() => client.closedDisputes)
const closedDisputes = computed(() => {
  return closedDisputesResult.value.isSuccess() ? closedDisputesResult.value.data : []
})

onMounted(async () => {
  nextTick(async () => {
    await client.fetchDisputedTrades()
  })
})

watch(userWallet, async () => {
  await client.fetchDisputedTrades()
})
</script>

<template>
  <section>
    <div class="dispute-list">
      <ul>
        <li v-for="dispute in openDisputes" :key="dispute.trade.id" class="card">
          <OpenDisputeItem :dispute="dispute" />
        </li>
      </ul>
    </div>

    <h3>Archived Disputes</h3>
    <div class="archived-disputes-table card">
      <div class="table-header">
        <div class="col-1">
          <p>Offer Type</p>
        </div>
        <div class="col-2">
          <p>Settle Date</p>
        </div>
        <div class="col-3">
          <p>Crypto</p>
        </div>
        <div class="col-4">
          <p>LOCAL Reward</p>
        </div>
        <div class="col-5">
          <p>Status</p>
        </div>
      </div>

      <div v-for="dispute in closedDisputes" :key="dispute.trade.id" class="wrap-table-item">
        <div class="col-1">
          <p>{{ dispute.offer.offer.offer_type }}</p>
        </div>
        <div class="col-2">
          <p>?????</p>
        </div>
        <div class="col-3">
          <p>{{ microDenomToDisplay(dispute.offer.offer.denom.native, client.chainClient) }}</p>
        </div>
        <div class="col-4">
          <p>?????</p>
        </div>
        <div class="col-5">
          <p>{{ formatTradeState(dispute.trade.state) }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

/* ----------- OFFER LIST */
.dispute-list {
  li {
    list-style: none;
    margin-bottom: 24px;

    &:last-child {
      margin-bottom: 56px;
    }
  }
}

/* ----------- ARCHIVED DISPUTES TABLE */
.archived-disputes-table {
  .table-header {
    border-bottom: 1px solid $border;
    margin-bottom: 16px;

    p {
      font-size: 14px;
      font-weight: $semi-bold;
      color: $gray700;
    }
  }
  .wrap-table-item {
    p {
      font-size: 14px;
      font-weight: $regular;
    }
  }

  .table-header,
  .wrap-table-item {
    display: flex;
    padding: 16px;
    flex-wrap: wrap;
  }

  .col-1,
  .col-5 {
    flex: 1 1 10%;
    text-transform: capitalize;
  }

  .col-2,
  .col-3,
  .col-4 {
    flex: 1 1 20%;
  }
  @media only screen and (max-width: $mobile) {
    min-width: 1000px;
    padding: 8px 0 16px 0;
  }
}
</style>
