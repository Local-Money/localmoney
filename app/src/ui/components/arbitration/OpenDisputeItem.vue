<script setup lang="ts">
// import type { TradeInfo } from '~/types/components.interface'

// const props = defineProps<{ dispute: TradeInfo }>()
</script>

<template>
  <div class="dispute-wrap">
    <div class="time"><p>45m ago</p></div>
    <div class="divider"></div>
    <div class="info">
      <div class="dispute-status">
        <div class="wrap">
          <p class="peer">Maker</p>
          <p class="address">kujira7...o8lkv</p>
        </div>
        <div class="wrap-offer-type">
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
          <p class="offer-type">
            Selling to
            <!-- {{ props.dispute.offer.offer_type }} -->
          </p>
        </div>
        <div class="wrap">
          <p class="peer">Taker</p>
          <p class="address">kujira7...o8lkv</p>
        </div>
      </div>
      <div class="divider"></div>
      <div class="wrap reward">
        <p class="label">Estimated rewards</p>
        <!-- TO-DO Get Trade Rate -->
        <p class="rate">$12.826 LOCAL</p>
      </div>
    </div>

    <div class="btn">
      <!-- <router-link :to="`/trade/${dispute.trade.id}`"> -->
      <button class="primary bg-gray300" type="button">view</button>
      <!-- </router-link> -->
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.dispute-wrap {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 40px;

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
    justify-content: space-between;
    align-items: center;
    flex-grow: 2;
    gap: 24px;

    .dispute-status {
      display: flex;
      align-items: center;
      gap: 40px;

      .wrap {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 16px;

        .peer {
          font-size: 16px;
        }
        .address {
          font-size: 12px;
          color: $gray600;
          background-color: $gray150;
          border-radius: 8px;
          padding: 4px 8px;
        }
      }

      .wrap-offer-type {
        display: flex;
        gap: 8px;
        align-items: center;
        flex-direction: row-reverse;
        .offer-type {
          font-size: 14px;
          font-weight: $semi-bold;
          color: $gray900;
        }
        .icon svg {
          vertical-align: middle;
          stroke: $primary;
        }
      }
    }

    .wrap {
      display: flex;
      flex-direction: column;
      .label {
        margin-bottom: 4px;
        font-size: 12px;
        color: $gray600;
      }

      .limit,
      .rate,
      .content {
        font-size: 15px;
        color: $gray700;
      }
    }

    .reward {
      text-align: right;
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
  .divider {
    height: 40px;
    width: 1px;
    background-color: $border;
  }
}
</style>
