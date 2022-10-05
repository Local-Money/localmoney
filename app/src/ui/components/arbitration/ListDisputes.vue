<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useClientStore } from '~/stores/client'
import { microDenomToDenom } from '~/utils/denom'

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
        <div class="col-6" />
      </div>
      <div v-for="dispute in closedDisputes" :key="dispute.trade.id" class="wrap-table-item">
        <div class="col-1">
          <p>{{ dispute.offer.offer_type }}</p>
        </div>
        <div class="col-2">
          <p>{{}}</p>
        </div>
        <div class="col-3">
          <p>{{ microDenomToDenom(dispute.offer.denom.native) }}</p>
        </div>
        <div class="col-4">
          <p>{{}}</p>
        </div>
        <div class="col-5">
          <p>{{ dispute.trade.state }}</p>
        </div>
        <div class="col-6"></div>
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
</style>
