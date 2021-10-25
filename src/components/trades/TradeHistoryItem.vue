<template>
  <div class="wrap-table-item">
    <div class="col-1"><p>18 Jul 2021</p></div>
    <div class="col-2"><p>Selling</p></div>
    <div class="col-3"><p>100 UST</p></div>
    <div class="col-4"><p>$365,900.00 COP</p></div>
    <div class="col-5 trader"><p>sambarboza</p></div>
    <div class="col-6"><p>Finalized</p></div>
  </div>
</template>

<script></script>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.wrap-table-item {
  display: flex;
  flex-direction: row;
  padding: 16px;

  p {
    font-size: 14px;
    font-weight: $regular;
  }
  .trader {
    color: $primary;
  }
}
</style>
