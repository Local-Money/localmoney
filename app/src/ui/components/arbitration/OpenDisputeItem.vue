<script setup lang="ts">
import { formatAddress } from '~/shared'
import type { TradeInfo } from '~/types/components.interface'
import { microDenomToDisplay } from '~/utils/denom'
import { useClientStore } from '~/stores/client'

const props = defineProps<{ dispute: TradeInfo }>()
const client = useClientStore()
const taker = computed(() => {
  const maker = props.dispute.offer.offer.owner
  const buyer = props.dispute.trade.buyer
  const seller = props.dispute.trade.seller
  return [buyer, seller].filter((it) => it !== maker)[0]
})
</script>

<template>
  <div class="dispute-wrap">
    <div class="time">
      <div class="icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
            stroke="inherit"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path d="M12 6V12L16 14" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <p>??m ago</p>
    </div>

    <div class="divider"></div>

    <div class="info">
      <div class="dispute-status">
        <div class="wrap-peer">
          <p class="peer">Maker</p>
          <p class="address">{{ formatAddress(props.dispute.offer.offer.owner) }}</p>
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
          <p v-if="props.dispute.offer.offer.offer_type === 'buy'" class="offer-type">
            buying <strong>{{ microDenomToDisplay(dispute.offer.offer.denom.native, client.chainClient) }}</strong> from
          </p>
          <p v-else class="offer-type">
            selling <strong>{{ microDenomToDisplay(dispute.offer.offer.denom.native, client.chainClient) }}</strong> to
          </p>
        </div>
        <div class="wrap-peer">
          <p class="peer">Taker</p>
          <p class="address">{{ formatAddress(taker) }}</p>
        </div>
      </div>
    </div>

    <div class="divider"></div>

    <div class="wrap-cta">
      <div class="reward">
        <p class="label">Estimated rewards</p>
        <!-- TO-DO Get Trade Rate -->
        <p class="rate">$??? {{ microDenomToDisplay(dispute.offer.offer.denom.native, client.chainClient) }}</p>
      </div>
      <router-link :to="`/trade/${dispute.trade.id}`">
        <button class="primary bg-gray300" type="button">view</button>
      </router-link>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.dispute-wrap {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 32px;

  @media only screen and (max-width: $mobile) {
    flex-direction: column;
    gap: 32px;
  }
  .time {
    display: flex;
    align-items: center;
    margin-left: 8px;

    @media only screen and (max-width: $mobile) {
      background-color: $gray150;
      padding: 8px 16px;
      border-radius: 8px;
    }

    .icon svg {
      vertical-align: middle;
      stroke: $gray700;
    }
    p {
      font-size: 16px;
      font-weight: $semi-bold;
      color: $primary;
      margin-left: 16px;
    }
  }

  .info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 24px;

    @media only screen and (max-width: $mobile) {
      width: 100%;
      flex-direction: column;
      gap: 8px;
    }

    .dispute-status {
      display: flex;
      align-items: center;
      gap: 40px;

      @media only screen and (max-width: $mobile) {
        width: 100%;
        justify-content: space-between;
        flex-direction: row;
        gap: 16px;
      }

      .wrap-peer {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 16px;
        background-color: $gray150;
        border-radius: 8px;
        padding: 8px 16px;

        @media only screen and (max-width: $mobile) {
          flex-direction: column;
          gap: 4px;
          padding: 8px 8px;
        }

        .peer {
          font-size: 16px;
        }
        .address {
          font-size: 12px;
          color: $gray600;
        }
      }

      .wrap-offer-type {
        display: flex;
        gap: 8px;
        align-items: center;
        flex-direction: row-reverse;

        @media only screen and (max-width: $mobile) {
          flex-direction: column-reverse;
        }
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
  }
  .divider {
    height: 40px;
    width: 1px;
    background-color: $border;

    @media only screen and (max-width: $mobile) {
      display: none;
    }
  }

  .wrap-cta {
    display: flex;
    align-items: center;
    gap: 32px;

    @media only screen and (max-width: $mobile) {
      width: 100%;
      justify-content: space-between;
      border-top: 1px solid $border;
      padding-top: 24px;
    }

    .reward {
      display: flex;
      flex-direction: column;
      text-align: right;

      @media only screen and (max-width: $mobile) {
        text-align: left;
      }
      .label {
        margin-bottom: 4px;
        font-size: 12px;
        color: $gray600;
      }
      .rate {
        font-size: 15px;
        color: $gray700;
      }
    }
  }
}
</style>
