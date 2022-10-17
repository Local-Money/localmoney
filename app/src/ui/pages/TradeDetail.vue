<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { ref } from 'vue'
import {
  addTelegramURLPrefix,
  calculateFiatPriceByRate,
  convertOfferRateToMarginRate,
  formatAddress,
  formatAmount,
  formatTradesCountInfo,
} from '~/shared'
import { useClientStore } from '~/stores/client'
import { usePriceStore } from '~/stores/price'
import { microDenomToDenom } from '~/utils/denom'
import { decryptData } from '~/utils/crypto'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
const priceStore = usePriceStore()
const tradeInfo = ref()
const buyerContact = ref('')
const sellerContact = ref('')
const secrets = computed(() => client.getSecrets())
let refreshInterval: NodeJS.Timer

const route = useRoute()
const walletAddress = computed(() => client.userWallet.address)
const stepOneChecked = computed(() => {
  return [
    'escrow_funded',
    'fiat_deposited',
    'escrow_disputed',
    'escrow_released',
    'settled_for_taker',
    'settled_for_maker',
  ].includes(tradeInfo.value.trade.state)
})
const stepTwoChecked = computed(() => {
  return ['fiat_deposited', 'escrow_disputed', 'escrow_released', 'settled_for_taker', 'settled_for_maker'].includes(
    tradeInfo.value.trade.state
  )
})
const stepThreeChecked = computed(() => {
  return ['escrow_released', 'settled_for_taker', 'settled_for_maker'].includes(tradeInfo.value.trade.state)
})
const isBuyer = computed(() => tradeInfo.value.trade.buyer === walletAddress.value)
const counterparty = computed(() => {
  const trade = tradeInfo.value.trade
  return walletAddress.value === trade.seller ? trade.buyer : trade.seller
})
const fiatCurrency = computed(() => tradeInfo.value.offer.offer.fiat_currency)
const usdRate = computed(() => priceStore.getPrice(fiatCurrency.value))
const fiatPriceByRate = computed(() => calculateFiatPriceByRate(usdRate.value, tradeInfo.value.offer.offer.rate))
const offerPrice = computed(() => `${fiatCurrency.value} ${formatAmount(fiatPriceByRate.value, false)}`)
const fiatAmountStr = computed(() => {
  const fiatAmount = formatAmount((parseInt(tradeInfo.value.trade.amount) / 1000000) * fiatPriceByRate.value, false)
  return `${fiatCurrency.value} ${fiatAmount}`
})
const marginRate = computed(() => convertOfferRateToMarginRate(tradeInfo.value.offer.offer.rate))
const counterpartyEncryptedContact = computed(() =>
  isBuyer.value ? tradeInfo.value.trade.seller_contact : tradeInfo.value.trade.buyer_contact
)
const isCounterpartyContactAvailable = computed(() => counterpartyEncryptedContact.value !== null)
const counterpartyContact = asyncComputed(async () => {
  const encryptedContact = counterpartyEncryptedContact.value
  const privateKey = secrets.value.privateKey
  if (isCounterpartyContactAvailable.value) {
    return await decryptData(privateKey, encryptedContact)
  } else {
    return 'pending ...'
  }
})

const isArbitrator = computed(() => {
  return client.arbitrators.data.filter((a) => client.userWallet.address === a.arbitrator).length > 0
})

const maker = computed(() => {
  return tradeInfo.value.offer.offer.owner
})

const taker = computed(() => {
  return tradeInfo.value.trade.buyer === maker ? tradeInfo.value.trade.seller : tradeInfo.value.trade.buyer
})
const contactsForArbitrator = computed(() => {
  const makerContact = tradeInfo.value.trade.buyer === maker.value ? buyerContact.value : sellerContact.value
  const takerContact = tradeInfo.value.trade.seller === maker.value ? buyerContact.value : sellerContact.value
  return {
    makerContact,
    takerContact,
  }
})

function fetchTrade(id: string) {
  nextTick(async () => {
    tradeInfo.value = await client.fetchTradeDetail(id)
    buyerContact.value = await decryptData(secrets.value.privateKey, tradeInfo.value.trade.arbitrator_buyer_contact)
    sellerContact.value = await decryptData(secrets.value.privateKey, tradeInfo.value.trade.arbitrator_seller_contact)
    console.log('tradeInfo.value', tradeInfo.value)
    refreshInterval = setInterval(async () => {
      tradeInfo.value = await client.fetchTradeDetail(id)
    }, 10 * 1000)
  })
}

onBeforeMount(async () => {
  fetchTrade(route.params.id as string)
})

onMounted(() => {})

onUnmounted(() => {
  clearInterval(refreshInterval)
})

watch(userWallet, async () => {
  return fetchTrade(route.params.id as string)
})
</script>

<template>
  <main v-if="tradeInfo" class="page">
    <h3 v-if="tradeInfo.trade.arbitrator === walletAddress">
      <template v-if="tradeInfo.trade.state === 'escrow_disputed'">Dispute in progress</template>
      <template v-if="tradeInfo.trade.state === 'settled_for_taker'">Dispute settled for taker</template>
      <template v-if="tradeInfo.trade.state === 'settled_for_maker'">Dispute settled for maker</template>
    </h3>
    <template v-else>
      <h3 v-if="isBuyer">
        Buying {{ microDenomToDenom(tradeInfo.trade.denom.native) }} from {{ formatAddress(counterparty) }}
      </h3>
      <h3 v-else>Selling {{ microDenomToDenom(tradeInfo.trade.denom.native) }} to {{ formatAddress(counterparty) }}</h3>
    </template>
    <section class="stepper card">
      <!-- Step 1 -->
      <div class="step-item">
        <IconDone v-if="stepOneChecked" />
        <div v-else class="icon">
          <div class="counter">
            <p>1</p>
          </div>
        </div>
        <p :class="stepOneChecked ? 'step-checked' : ''">waiting for funds</p>
      </div>

      <!-- Step 2 -->
      <div class="step-item">
        <IconDone v-if="stepTwoChecked" />
        <div v-else class="icon">
          <div class="counter">
            <p>2</p>
          </div>
        </div>
        <p :class="stepTwoChecked ? 'step-checked' : ''">waiting for payment</p>
      </div>

      <!-- Step 3 -->
      <div class="step-item">
        <IconDone v-if="stepThreeChecked" />
        <div v-else class="icon">
          <div class="counter">
            <p>3</p>
          </div>
        </div>
        <p
          v-if="['escrow_disputed', 'settled_for_taker', 'settled_for_maker'].includes(tradeInfo.trade.state)"
          :class="['settled_for_taker', 'settled_for_maker'].includes(tradeInfo.trade.state) ? 'step-checked' : ''"
        >
          in dispute
        </p>
        <p v-else :class="stepThreeChecked ? 'step-checked' : ''">waiting for funds release</p>
      </div>

      <div class="step-status">
        <div class="separator" />
        <div class="wrap">
          <p>time remaining</p>
          <p class="step-time-left">?? min</p>
        </div>
        <div class="icon">
          <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none">
            <path
              d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path d="M12 6V12L16 14" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
      </div>
    </section>

    <section class="wrap">
      <!-- ChatboxStates -->
      <div class="chat card">
        <div class="content">
          <div v-if="!isArbitrator">
            <!-- Contact available -->
            <div v-if="isCounterpartyContactAvailable" class="contact-available">
              <div class="trader-info">
                <p><small>You're trading with</small></p>
                <p class="trader">
                  {{ formatAddress(counterparty) }}
                </p>
                <p class="rating">{{ formatTradesCountInfo(tradeInfo.offer.profile.trades_count) }}</p>
              </div>
              <p class="guide-content">
                Open a chat with the other trader so you can exchange information about payment.
              </p>
              <div class="contact-info">
                <a :href="addTelegramURLPrefix(counterpartyContact)" class="btn-telegram" target="_blank">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path
                      fill-rule="evenodd"
                      clip-rule="evenodd"
                      d="M3.30615 11.0109C8.40641 8.7126 11.8074 7.19743 13.5091 6.46537C18.3677 4.3752 19.3773 4.01212 20.0354 4.00013C20.1801 3.99749 20.5037 4.03459 20.7133 4.21051C20.8903 4.35905 20.939 4.55971 20.9623 4.70055C20.9856 4.84139 21.0146 5.16221 20.9916 5.4129C20.7283 8.27419 19.589 15.2178 19.0094 18.4225C18.7642 19.7785 18.2813 20.2331 17.8138 20.2776C16.7978 20.3743 16.0263 19.5832 15.0422 18.916C13.5024 17.872 12.6325 17.2222 11.1378 16.2034C9.4105 15.0261 10.5303 14.379 11.5147 13.3215C11.7723 13.0448 16.2488 8.83347 16.3354 8.45144C16.3463 8.40366 16.3563 8.22556 16.254 8.13152C16.1517 8.03748 16.0007 8.06964 15.8918 8.09521C15.7373 8.13147 13.2775 9.81311 8.51212 13.1401C7.81389 13.636 7.18145 13.8776 6.61481 13.865C5.99014 13.851 4.78851 13.4997 3.89523 13.1993C2.79958 12.831 1.92878 12.6362 2.0046 12.0106C2.0441 11.6848 2.47795 11.3515 3.30615 11.0109Z"
                      fill="inherit"
                    />
                  </svg>
                  <p>open chat</p>
                </a>
              </div>
            </div>
            <!-- End Contact available -->

            <!-- Contact waiting -->
            <div v-else class="contact-waiting">
              <p class="guide-content">Once the trade starts you will be able to contact the other trader.</p>
            </div>
            <!-- End Contact waiting -->
          </div>
          <div v-else>
            <div class="title">
              <p>Contact information</p>
            </div>
            <p class="guide-content">Please contact both parties to begin the dispute resolution process.</p>

            <a :href="addTelegramURLPrefix(contactsForArbitrator.makerContact)" class="btn-telegram" target="_blank">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M3.30615 11.0109C8.40641 8.7126 11.8074 7.19743 13.5091 6.46537C18.3677 4.3752 19.3773 4.01212 20.0354 4.00013C20.1801 3.99749 20.5037 4.03459 20.7133 4.21051C20.8903 4.35905 20.939 4.55971 20.9623 4.70055C20.9856 4.84139 21.0146 5.16221 20.9916 5.4129C20.7283 8.27419 19.589 15.2178 19.0094 18.4225C18.7642 19.7785 18.2813 20.2331 17.8138 20.2776C16.7978 20.3743 16.0263 19.5832 15.0422 18.916C13.5024 17.872 12.6325 17.2222 11.1378 16.2034C9.4105 15.0261 10.5303 14.379 11.5147 13.3215C11.7723 13.0448 16.2488 8.83347 16.3354 8.45144C16.3463 8.40366 16.3563 8.22556 16.254 8.13152C16.1517 8.03748 16.0007 8.06964 15.8918 8.09521C15.7373 8.13147 13.2775 9.81311 8.51212 13.1401C7.81389 13.636 7.18145 13.8776 6.61481 13.865C5.99014 13.851 4.78851 13.4997 3.89523 13.1993C2.79958 12.831 1.92878 12.6362 2.0046 12.0106C2.0441 11.6848 2.47795 11.3515 3.30615 11.0109Z"
                  fill="inherit"
                />
              </svg>
              <p>chat with maker</p>
            </a>

            <a :href="addTelegramURLPrefix(contactsForArbitrator.takerContact)" class="btn-telegram" target="_blank">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M3.30615 11.0109C8.40641 8.7126 11.8074 7.19743 13.5091 6.46537C18.3677 4.3752 19.3773 4.01212 20.0354 4.00013C20.1801 3.99749 20.5037 4.03459 20.7133 4.21051C20.8903 4.35905 20.939 4.55971 20.9623 4.70055C20.9856 4.84139 21.0146 5.16221 20.9916 5.4129C20.7283 8.27419 19.589 15.2178 19.0094 18.4225C18.7642 19.7785 18.2813 20.2331 17.8138 20.2776C16.7978 20.3743 16.0263 19.5832 15.0422 18.916C13.5024 17.872 12.6325 17.2222 11.1378 16.2034C9.4105 15.0261 10.5303 14.379 11.5147 13.3215C11.7723 13.0448 16.2488 8.83347 16.3354 8.45144C16.3463 8.40366 16.3563 8.22556 16.254 8.13152C16.1517 8.03748 16.0007 8.06964 15.8918 8.09521C15.7373 8.13147 13.2775 9.81311 8.51212 13.1401C7.81389 13.636 7.18145 13.8776 6.61481 13.865C5.99014 13.851 4.78851 13.4997 3.89523 13.1993C2.79958 12.831 1.92878 12.6362 2.0046 12.0106C2.0441 11.6848 2.47795 11.3515 3.30615 11.0109Z"
                  fill="inherit"
                />
              </svg>
              <p>chat with taker</p>
            </a>
          </div>
        </div>
      </div>
      <!-- End ChatboxStates -->

      <div class="summary">
        <!-- Trade Summary -->
        <div v-if="!isArbitrator" class="trade-summary card">
          <div class="trade-info">
            <p class="label">Price</p>
            <div class="current-price">
              <p class="mkt-rate">{{ marginRate.marginOffset }}% {{ marginRate.margin }} market</p>
              <p class="price">
                {{ offerPrice }}
              </p>
            </div>
            <p class="label">Transaction summary</p>
            <div class="transaction">
              <div class="list-item">
                <p v-if="isBuyer" class="list-item-label">You will get</p>
                <p v-else class="list-item-label">You will send</p>
                <p class="value">
                  {{ formatAmount(tradeInfo.trade.amount) }} {{ microDenomToDenom(tradeInfo.trade.denom.native) }}
                </p>
              </div>
              <div class="list-item">
                <p v-if="isBuyer" class="list-item-label">You will send</p>
                <p v-else class="list-item-label">You will get</p>
                <p class="value fiat">
                  {{ fiatAmountStr }}
                </p>
              </div>
            </div>
          </div>
        </div>
        <!-- End Trade Summary -->

        <!-- Trade Dispute Summary -->
        <div v-else class="dispute-summary card">
          <div class="dispute-wrap">
            <div class="peer-wrap">
              <p class="peer">Maker</p>
              <p class="address">{{ formatAddress(maker) }}</p>
            </div>
            <div class="separator">
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
            <div class="peer-wrap">
              <p class="peer">Taker</p>
              <p class="address">{{ formatAddress(taker) }}</p>
            </div>
          </div>
          <div class="trade-info">
            <p class="label">Transaction summary</p>
            <div class="transaction">
              <div class="list-item">
                <p v-if="tradeInfo.offer.offer_type === 'sell'" class="list-item-label">Maker is selling</p>
                <p v-else class="list-item-label">Maker is buying</p>
                <p class="value">
                  {{ formatAmount(tradeInfo.trade.amount) }} {{ microDenomToDenom(tradeInfo.trade.denom.native) }}
                </p>
              </div>
              <div class="list-item">
                <p v-if="tradeInfo.offer.offer_type === 'sell'" class="list-item-label">Taker should pay</p>
                <p v-else class="list-item-label">Taker should receive</p>
                <p class="value fiat">
                  {{ fiatAmountStr }}
                </p>
              </div>
            </div>
          </div>
        </div>
        <!-- End Trade Dispute Summary -->
        <TradeActions :tradeInfo="tradeInfo" :walletAddress="walletAddress" />
      </div>
    </section>
  </main>
  <main v-else class="page">
    <div v-if="!client.userWallet.isConnected" class="error-state card">
      <p>You need to connect your wallet</p>
    </div>
    <div v-else class="loading-state">
      <Loading />
    </div>
  </main>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

.stepper {
  display: flex;
  justify-content: space-between;
  padding: 24px 40px;
  margin-bottom: 24px;
}

.step-item,
.step-status {
  width: 20%;
  display: flex;
  align-items: center;
}

.step-item {
  .icon {
    margin-right: 24px;
  }

  .counter {
    width: 32px;
    height: 32px;
    border-radius: 100px;
    background-color: $border;
    text-align: center;
    padding-top: 6px;
    font-size: 14px;
    font-weight: $semi-bold;
  }

  p {
    font-size: 14px;
  }

  .step-checked {
    opacity: 0.3;
  }
}

.step-status {
  justify-content: flex-end;
  border-left: 1px solid $border;

  .wrap {
    text-align: right;
    margin-right: 24px;
    display: flex;
    flex-direction: column;

    p {
      font-size: 14px;
      color: $gray900;
    }

    .step-time-left {
      font-size: 18px;
      font-weight: $semi-bold;
      color: $primary;
    }
  }
}

.error-state {
  display: flex;
  justify-content: center;
  margin-top: 32px;
}

.loading-state {
  display: flex;
  justify-content: center;
  margin-top: 92px;
}

.wrap {
  display: flex;
}

.main-wrap {
  display: flex;
}

.chat {
  flex: 1;
  margin-right: 24px;
  margin-bottom: 64px;

  .trader-info,
  .title {
    padding-bottom: 24px;
    margin-bottom: 24px;
    border-bottom: 1px solid $border;
    .trader,
    p {
      font-size: 18px;
      font-weight: $semi-bold;
    }

    .rating {
      font-size: 14px;
      color: $gray700;
      margin-top: 8px;
    }
  }

  .content {
    .guide-content {
      color: $gray700;
      font-size: 14px;
    }

    .contact {
      color: $gray900;
    }

    .telegram {
      color: $primary;

      &:hover {
        color: $secondary;
      }
    }
    .btn-telegram {
      display: inline-flex;
      flex-grow: 1;
      gap: 8px;
      align-items: center;
      padding: 8px 16px;
      border-radius: 100px;
      margin-top: 24px;
      background-color: #229ed9;
      color: $base-text;
      font-size: 14px;
      font-weight: $semi-bold;

      svg {
        width: 16px;
        height: 16px;
        fill: $base-text;
      }
    }
  }
}

.summary {
  display: flex;
  flex-direction: column;
  flex: 2.5;
}

.dispute-summary {
  flex-direction: column;

  .dispute-wrap {
    display: flex;
    justify-content: space-around;
    align-items: center;
    margin-bottom: 40px;

    .peer-wrap {
      text-align: center;

      .peer {
        font-size: 20px;
        font-weight: $semi-bold;
        margin-bottom: 8px;
      }
      .address {
        font-size: 14px;
        background-color: $gray300;
        border-radius: 8px;
        padding: 4px 16px;
      }
    }
    .separator svg {
      stroke: $primary;
    }
  }
}

.trade-summary,
.dispute-summary {
  display: flex;

  .label {
    margin-bottom: 8px;
    font-size: 14px;
    color: $gray900;
  }

  .trade-info {
    width: 100%;

    .current-price,
    .transaction {
      background-color: $gray150;
      padding: 16px;
      border-radius: 8px;
    }

    .current-price {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 24px;

      .mkt-rate {
        font-size: 14px;
        color: $gray900;
      }

      .price {
        font-size: 16px;
        font-weight: $semi-bold;
      }
    }

    .transaction {
      .list-item {
        display: flex;
        justify-content: space-between;
        align-items: center;

        &:first-child {
          margin-bottom: 8px;
        }

        p {
          font-size: 16px;
        }

        .value {
          font-weight: $semi-bold;
        }

        .fiat {
          color: $primary;
        }
      }
    }
  }
}
</style>
