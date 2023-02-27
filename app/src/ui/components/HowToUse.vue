<script setup lang="ts"></script>

<template>
  <section class="page">
    <div class="wrap">
      <div class="item">
        <div class="inner-wrap">
          <p class="title">How to trade</p>
          <p class="sub-title">learn more</p>
        </div>
        <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M9 18L15 12L9 6" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <div class="item">
        <div class="inner-wrap">
          <p class="title">Become a maker</p>
          <p class="sub-title">learn more</p>
        </div>
        <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M9 18L15 12L9 6" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <div class="item">
        <div class="inner-wrap">
          <p class="title primary">Get some $LOCAL</p>
          <p class="sub-title">Go to FIN</p>
        </div>
        <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M9 18L15 12L9 6" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
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
  gap: 24px;
  margin: 24px 0 16px;
  padding-bottom: 64px;
  border-bottom: 1px solid $border;

  .item {
    width: 33%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 24px;
    cursor: pointer;
    background-color: $gray150;
    border: 1px solid $border;
    border-radius: 8px;
    box-shadow: 0px 2px 25px rgba(0, 0, 0, 0.05);

    &:hover {
      background-color: $surface;
    }
    .title {
      margin-bottom: 8px;
      font-size: 16px;
      line-height: 16px;
      font-weight: $semi-bold;
    }
    .sub-title {
      font-size: 14px;
      line-height: 14px;
      font-weight: $regular;
      color: $gray700;
    }
    .primary {
      color: $primary;
    }

    svg {
      stroke: $border;
    }
  }
}
</style>
