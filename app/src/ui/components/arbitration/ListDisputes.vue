<script setup lang="ts"></script>

<template>
  <section>
    <div class="dispute-list">
      <ul>
        <li class="card">
          <OpenDisputeItem />
        </li>
      </ul>
    </div>

    <h3>Archived Offers</h3>
    <div class="archived-disputes-table card">
      <div class="table-header">
        <div class="col-1">
          <p>Order Type</p>
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
