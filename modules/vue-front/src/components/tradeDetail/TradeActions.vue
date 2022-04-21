<template>
  <section class="actions card">
    <!-- # If the user is buying UST (Buyer) -->
    <div v-if="isBuyer && !this.tradeInfo.expired">
      <!-- #1 step (Optional) -->
      <!-- # A Seller requested a trade with the Buyer and it should be accepted first. -->
      <TradeAction
          v-if="tradeInfo.offer.offer_type === 'buy' && tradeInfo.trade.state === 'request_created'"
          :message="'Review the request and accept the trade'"
          :button-label="'accept trade'"
          @actionClick="this.acceptTradeRequest(tradeInfo.trade.addr)"
      />
      <!-- #2 step or #1 step-->
      <!-- if #2 step: The Buyer accepted the request and needs to wait for the Seller to deposit UST on escrow-->
      <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the UST on escrow-->
      <TradeAction
          v-if="(tradeInfo.offer.offer_type === 'sell' && tradeInfo.trade.state === 'request_created') ||
                (tradeInfo.trade.state === 'request_accepted')"
          :message="'Waiting for the trade to be funded'"
      />
      <!-- #3 step or #2 step-->
      <!-- The UST is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
      <TradeAction
          v-if="tradeInfo.trade.state === 'escrow_funded'"
          :message="'Only press the mark as paid after you made the payment'"
          :button-label="'mark as paid'"
          @actionClick="this.setFiatDeposited(tradeInfo.trade.addr)"
      />
      <!-- #4 step or #3 step-->
      <!-- After the off-chain payment, the Buyer needs to wait for the Seller to release the funds on escrow -->
      <TradeAction
          v-if="tradeInfo.trade.state === 'fiat_deposited'"
          :message="'Waiting for funds to be released'"
      />
      <!-- #5 step or #4 step-->
      <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
      <TradeAction
          v-if="tradeInfo.trade.state === 'escrow_released'"
          :message="'Trade finished successfully'"
      />
    </div>

    <!-- # If the user is selling UST (Seller) -->
    <div v-if="isSeller && !this.tradeInfo.expired">
      <!-- #1 step (Optional) -->
      <!-- # The Seller opens the trade with the Buyer and it should be accepted first. So the Seller needs to wait. -->
      <TradeAction
          v-if="this.tradeInfo.offer.offer_type === 'buy' && tradeInfo.trade.state === 'request_created'"
          :message="'Waiting for the buyer to accept the trade'"
      />
      <!-- #2 step or #1 step-->
      <!-- if #2 step: The Seller needs to deposit UST on escrow to enable the Buyer to transfer the Fiat-->
      <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the UST on escrow-->
      <TradeAction
          v-if="(this.tradeInfo.offer.offer_type === 'sell' && tradeInfo.trade.state === 'request_created') ||
                (tradeInfo.trade.state === 'request_accepted')"
          :message="'Please fund the trade'"
          :button-label="'fund trade'"
          @actionClick="this.fundEscrow(tradeInfo.trade.addr)"
      />
      <!-- #3 step or #2 step-->
      <!-- The UST is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
      <TradeAction
          v-if="tradeInfo.trade.state === 'escrow_funded'"
          :message="'Waiting for payment from the buyer'"
      />
      <!-- #4 step or #3 step-->
      <!-- After the off-chain payment, the Seller needs to check the off-chain payment and release the UST on the escrow to the Buyer -->
      <TradeAction
          v-if="tradeInfo.trade.state === 'fiat_deposited'"
          :message="'Only release the funds after confirming the payment'"
          :button-label="'release funds'"
          @actionClick="this.releaseEscrow(tradeInfo.trade.addr)"
      />
      <!-- #5 step or #4 step-->
      <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
      <TradeAction
          v-if="tradeInfo.trade.state === 'escrow_released'"
          :message="'Trade finished successfully'"
      />
    </div>
    <!-- Trade expired -->
    <!-- TODO the expired will change to a TradeState-->
    <TradeAction
        v-if="this.tradeInfo.expired && this.tradeInfo.trade.state !== 'escrow_refunded'"
        :message="'This trade has expired'"
    />

    <!-- Trade refunded -->
    <!-- TODO the expired will change to a TradeState-->
    <TradeAction
        v-if="this.tradeInfo.expired && this.tradeInfo.trade.state === 'escrow_refunded'"
        :message="'The funds have been refunded'"
    />

    <!-- Trade canceled -->
    <TradeAction
        v-if="tradeInfo.trade.state === 'request_canceled'  && !this.tradeInfo.expired"
        :message="'This trade has been canceled'"
    />
  </section>

  <section class="wrap sub-actions">
    <button
        class="tertiary"
        v-if="(tradeInfo.trade.state === 'request_created' ||
        tradeInfo.trade.state === 'request_accepted' ||
        (tradeInfo.trade.state === 'escrow_funded' && isBuyer)) && !tradeInfo.expired"
        @click="this.cancelTradeRequest(tradeInfo.trade.addr)"
    >
      cancel
    </button>

    <button
        class="tertiary"
        v-if="isSeller && tradeInfo.trade.state === 'escrow_funded' && tradeInfo.expired"
        @click="this.refundEscrow(tradeInfo.trade.addr)"
    >
      refund escrow
    </button>

    <button class="tertiary"
        v-if="tradeInfo.trade.state === 'fiat_deposited'"
        @click="this.openDispute(tradeInfo.trade.addr)" disabled
    >
      open dispute
    </button>
  </section>
</template>

<script>
import TradeAction from "@/components/tradeDetail/TradeAction"
import { defineComponent } from "vue";
import { mapActions } from "vuex";

export default  defineComponent({
  name: 'TradeActions',
  components: {TradeAction},
  props: {
    tradeInfo: {
      type: Object,
      required: true
    },
    walletAddress: {
      type: String,
      required: true
    }
  },
  methods: {
    ...mapActions([
      "acceptTradeRequest",
      "cancelTradeRequest",
      "fundEscrow",
      "setFiatDeposited",
      "releaseEscrow",
      "refundEscrow",
      "openDispute",
    ]),
  },
  computed: {
    isBuyer: function () {
      return this.tradeInfo.trade.buyer === this.walletAddress
    },
    isSeller: function () {
      return this.tradeInfo.trade.seller === this.walletAddress
    },
  }
})
</script>
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