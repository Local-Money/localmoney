<script setup lang="ts"></script>

<template>
  <div class="dispute-wrap">
    <div class="time"><p>45m ago</p></div>
    <div class="info">
      <div class="wrap">
        <p class="offer-type">Buy Order</p>
      </div>

      <div class="divider" />
      <div class="wrap">
        <p class="label">Reward (estimated)</p>
        <!-- TO-DO Get Trade Rate -->
        <p class="rate">$12.826 LOCAL</p>
      </div>
    </div>

    <div class="dispute-status">
      <div class="wrap">
        <!-- <p class="peer">Maker</p> -->
        <p class="address">kujira7...o8lkv</p>
      </div>
      <div class="icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M5 12H19" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          <path
            d="M12 5L19 12L12 19"
            stroke="inherit"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
      <div class="wrap">
        <!-- <p class="peer">Taker</p> -->
        <p class="address">kujira7...o8lkv</p>
      </div>
    </div>

    <div class="btn">
      <button class="primary bg-gray300" type="button">view</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.dispute-wrap {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;

  @media only screen and (max-width: $mobile) {
    flex-direction: column;
    gap: 24px;
  }
  .time p {
    font-size: 16px;
    font-weight: $semi-bold;
    color: $primary;
    background-color: $gray150;
    border-radius: 8px;
    padding: 8px 16px;
  }

  .info {
    display: flex;
    align-items: center;
    gap: 24px;

    .divider {
      height: 40px;
      width: 1px;
      background-color: $border;
    }

    .wrap {
      display: flex;
      flex-direction: column;
      .label {
        margin-bottom: 4px;
        font-size: 12px;
        color: $gray600;
      }
      .offer-type {
        font-size: 14px;
        font-weight: $semi-bold;
        color: $gray900;
      }

      .limit,
      .rate,
      .content {
        font-size: 15px;
        color: $gray700;
      }
    }

    @media only screen and (max-width: $mobile) {
      flex-direction: column;
      align-items: flex-start;
      gap: 8px;
      background-color: $gray150;
      padding: 8px 16px;
      border-radius: 8px;

      .divider {
        display: none;
      }
      .wrap {
        width: 100%;
        flex-direction: row;
        align-items: flex-end;
        gap: 24px;
        justify-content: space-between;
      }
    }
  }

  .dispute-status {
    display: flex;
    align-items: center;
    gap: 24px;

    .wrap {
      display: flex;
      flex-direction: column;
      align-items: center;

      .peer {
        font-size: 14px;
        margin-bottom: 4px;
      }
      .address {
        font-size: 14px;
        background-color: $gray150;
        border-radius: 8px;
        padding: 8px 16px;
      }
    }
    .icon svg {
      stroke: $primary;
    }
  }
}
</style>
