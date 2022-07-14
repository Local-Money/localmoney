<script setup lang="ts">
import type { TradeInfo } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'

const props = defineProps<{
  tradeInfo: TradeInfo
  walletAddress: String
}>()
const client = useClientStore()

const isBuyer = computed(() => props.tradeInfo.trade.buyer === props.walletAddress)
const isSeller = computed(() => props.tradeInfo.trade.seller === props.walletAddress)

async function acceptTradeRequest(id: string) {
  await client.acceptTradeRequest(id)
}

async function cancelTradeRequest(id: string) {
  await client.cancelTradeRequest(id)
}

async function fundEscrow(id: string) {
  await client.fundEscrow(id, props.tradeInfo.trade.amount, props.tradeInfo.trade.denom)
}

async function setFiatDeposited(id: string) {
  await client.setFiatDeposited(id)
}

async function releaseEscrow(id: string) {
  await client.releaseEscrow(id)
}

async function refundEscrow(id: string) {
  await client.refundEscrow(id)
}

async function openDispute(id: string) {
  await client.openDispute(id)
}
</script>

<template>
  <section class="actions card">
    <!-- # If the user is buying crypto (Buyer) -->
    <div v-if="isBuyer && !tradeInfo.expired">
      <!-- #1 step (Optional) -->
      <!-- # A Seller requested a trade with the Buyer and it should be accepted first. -->
      <TradeAction
        v-if="tradeInfo.offer.offer_type === 'buy' && tradeInfo.trade.state === 'request_created'"
        message="Review the request and accept the trade"
        button-label="accept trade"
        @actionClick="acceptTradeRequest(tradeInfo.trade.id)"
      />
      <!-- #2 step or #1 step -->
      <!-- if #2 step: The Buyer accepted the request and needs to wait for the Seller to deposit crypto on escrow -->
      <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the crypto on escrow -->
      <TradeAction
        v-if="(tradeInfo.offer.offer_type === 'sell' && tradeInfo.trade.state === 'request_created')
          || (tradeInfo.trade.state === 'request_accepted')"
        message="Waiting for the trade to be funded"
      />
      <!-- #3 step or #2 step -->
      <!-- The crypto is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_funded'"
        message="Only press the mark as paid after you made the payment"
        button-label="mark as paid"
        @actionClick="setFiatDeposited(tradeInfo.trade.id)"
      />
      <!-- #4 step or #3 step -->
      <!-- After the off-chain payment, the Buyer needs to wait for the Seller to release the funds on escrow -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'fiat_deposited'"
        message="Waiting for funds to be released"
      />
      <!-- #5 step or #4 step -->
      <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_released'"
        message="Trade finished successfully"
      />
    </div>

    <!-- # If the user is selling crypto (Seller) -->
    <div v-if="isSeller && !tradeInfo.expired">
      <!-- #1 step (Optional) -->
      <!-- # The Seller opens the trade with the Buyer and it should be accepted first. So the Seller needs to wait. -->
      <TradeAction
        v-if="tradeInfo.offer.offer_type === 'buy' && tradeInfo.trade.state === 'request_created'"
        message="Waiting for the buyer to accept the trade"
      />
      <!-- #2 step or #1 step -->
      <!-- if #2 step: The Seller needs to deposit crypto on escrow to enable the Buyer to transfer the Fiat -->
      <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the crypto on escrow -->
      <TradeAction
        v-if="(tradeInfo.offer.offer_type === 'sell' && tradeInfo.trade.state === 'request_created')
          || (tradeInfo.trade.state === 'request_accepted')"
        message="Please fund the trade"
        button-label="fund trade"
        @actionClick="fundEscrow(tradeInfo.trade.id)"
      />
      <!-- #3 step or #2 step -->
      <!-- The crypto is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_funded'"
        message="Waiting for payment from the buyer"
      />
      <!-- #4 step or #3 step -->
      <!-- After the off-chain payment, the Seller needs to check the off-chain payment and release the crypto on the escrow to the Buyer -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'fiat_deposited'"
        message="Only release the funds after confirming the payment"
        button-label="release funds"
        @actionClick="releaseEscrow(tradeInfo.trade.id)"
      />
      <!-- #5 step or #4 step -->
      <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_released'"
        message="Trade finished successfully"
      />
    </div>
    <!-- Trade expired -->
    <!-- TODO the expired will change to a TradeState -->
    <TradeAction
      v-if="tradeInfo.expired && tradeInfo.trade.state !== 'escrow_refunded'"
      message="This trade has expired"
    />

    <!-- Trade refunded -->
    <!-- TODO the expired will change to a TradeState -->
    <TradeAction
      v-if="tradeInfo.expired && tradeInfo.trade.state === 'escrow_refunded'"
      message="The funds have been refunded"
    />

    <!-- Trade canceled -->
    <TradeAction
      v-if="tradeInfo.trade.state === 'request_canceled' && !tradeInfo.expired"
      message="This trade has been canceled"
    />
  </section>

  <section class="wrap sub-actions">
    <button
      v-if="(tradeInfo.trade.state === 'request_created'
        || tradeInfo.trade.state === 'request_accepted'
        || (tradeInfo.trade.state === 'escrow_funded' && isBuyer)) && !tradeInfo.expired"
      class="tertiary"
      @click="cancelTradeRequest(tradeInfo.trade.id)"
    >
      cancel
    </button>

    <button
      v-if="isSeller && tradeInfo.trade.state === 'escrow_funded' && tradeInfo.expired"
      class="tertiary"
      @click="refundEscrow(tradeInfo.trade.id)"
    >
      refund escrow
    </button>

    <button
      v-if="tradeInfo.trade.state === 'fiat_deposited'"
      class="tertiary"
      disabled @click="openDispute(tradeInfo.trade.id)"
    >
      open dispute
    </button>
  </section>
</template>

<style lang="scss" scoped>
@import "../../style/pages";

.actions {
  margin-top: 24px;
}

.sub-actions {
  height: 64px;
}

button {
  background-color: $gray300;
  color: $primary;
  margin-left: auto;
  margin-top: 24px;
}
</style>
