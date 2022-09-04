<script setup lang="ts"></script>

<template>
  <div class="offer collapsed">
    <div class="time">45m ago</div>
    <div class="info">
      <div class="wrap">
        <p class="label">Limits</p>
        <!-- TO-DO Get Trade Limit -->
        <p class="limit">5.00 - 10.00 KUJI</p>
      </div>

      <div class="divider" />
      <div class="wrap">
        <p class="label">Price</p>
        <!-- TO-DO Get Trade Rate -->
        <p class="rate">2% above market</p>
      </div>
    </div>

    <div class="price">
      <!-- TO-DO Get Trade Price -->
      <p class="value">300</p>
      <button class="primary bg-gray300" type="button">view</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.collapsed {
  .owner {
    display: flex;
    flex-direction: column;

    @media only screen and (max-width: $mobile) {
      flex-direction: row;
      justify-content: space-between;
    }

    .wallet-addr {
      font-size: 18px;
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
