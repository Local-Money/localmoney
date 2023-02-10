<script setup lang="ts"></script>

<template>
  <section class="page">
    <div class="wrap">
      <div class="item card">
        <p class="title">Buy or Sell</p>
        <p class="content">
          Whether you want to buy or sell tokens, here in the marketplace you can find offers created by the community
          and choose the one that best suits your needs.
        </p>
      </div>
      <div class="item card">
        <p class="title">Escrow and FIAT payment</p>
        <p class="content">
          We are a non-custodial decentralized platform so we don’t hold any tokens. During the trade they are stored in
          an escrow (on-chain) and will be released after both sides fulfill their end of the trade.
        </p>
      </div>
      <div class="item card">
        <p class="title">Step 01</p>
        <p class="content">
          Lorem ipsum dolor sit amet consectetur adipisicing elit. Non natus eligendi id veniam reiciendis, itaque,
          pariatur unde aperiam ut labore recusandae et, sint fuga? Nemo cumque quod sint ipsa saepe?
        </p>
      </div>
    </div>
  </section>
</template>

<style lang="scss" scoped>
@import '../style/tokens.scss';

section {
  margin-top: 0;
}
.wrap {
  display: flex;
  gap: 32px;
  .item {
    .title {
      margin-bottom: 8px;
      font-size: 18px;
      font-weight: $semi-bold;
    }
    .content {
      font-size: 15px;
      font-weight: $regular;
      color: $gray900;
    }
  }
}
</style>
