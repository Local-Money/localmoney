<script setup lang="ts">
import type { TradeInfo } from '~/types/components.interface'
import { TradeState } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { formatAddress } from '~/shared'
import { decryptData, encryptData } from '~/utils/crypto'
import { formatTimer } from '~/utils/formatters'

const props = defineProps<{
  tradeInfo: TradeInfo
  walletAddress: String
}>()
const client = useClientStore()
const secrets = computed(() => client.getSecrets())
const profile = computed(() => client.profile)

const isBuyer = computed(() => props.tradeInfo.trade.buyer === props.walletAddress)
const isSeller = computed(() => props.tradeInfo.trade.seller === props.walletAddress)

const timeToEnableDispute = computed(() => {
  const currentTime = Date.now()
  const enablesDisputeAt = (props.tradeInfo.trade.enables_dispute_at ?? 0) * 1000
  return new Date(enablesDisputeAt - currentTime)
})

const disputeWinner = computed(() => {
  const taker = `${formatAddress(getTaker())}`
  const maker = `${formatAddress(getMaker())}`
  const winner = props.tradeInfo.trade.state === 'settled_for_taker' ? taker : maker
  return `${winner}`
})

const lastTradeState = computed(() => {
  const lastIndex = props.tradeInfo.trade.state_history.length - 1
  return props.tradeInfo.trade.state_history[lastIndex].state
})

function getMaker(): string {
  return props.tradeInfo.offer.offer.owner
}

function getTaker(): string {
  const maker = getMaker()
  return props.tradeInfo.trade.buyer === maker ? props.tradeInfo.trade.seller : props.tradeInfo.trade.buyer
}

async function acceptTradeRequest(id: number) {
  const takerPubKey = props.tradeInfo.trade.seller_encryption_key
  const decryptedContact = await decryptData(secrets.value.privateKey, profile.value.contact!)
  const ownerContact = await encryptData(takerPubKey, decryptedContact)
  await client.acceptTradeRequest(id, ownerContact)
}

async function cancelTradeRequest(id: number) {
  await client.cancelTradeRequest(id)
}

async function fundEscrow(tradeInfo: TradeInfo) {
  const buyerPubKey = props.tradeInfo.trade.buyer_encryption_key!
  const decryptedContact = await decryptData(secrets.value.privateKey, profile.value.contact!)
  const ownerContact = await encryptData(buyerPubKey, decryptedContact)
  await client.fundEscrow(tradeInfo, ownerContact)
}

async function setFiatDeposited(id: number) {
  await client.setFiatDeposited(id)
}

async function releaseEscrow(id: number) {
  await client.releaseEscrow(id)
}

async function refundEscrow(id: number) {
  await client.refundEscrow(id)
}

async function openDispute(id: number) {
  let buyerContact = ''
  let sellerContact = ''
  const userDecryptedContact = await decryptData(secrets.value.privateKey, profile.value.contact!)
  if (isBuyer.value) {
    const sellerDecryptedContact = await decryptData(secrets.value.privateKey, props.tradeInfo.trade.seller_contact!)
    buyerContact = await encryptData(props.tradeInfo.trade.arbitrator_encryption_key, userDecryptedContact)
    sellerContact = await encryptData(props.tradeInfo.trade.arbitrator_encryption_key, sellerDecryptedContact)
  } else {
    const buyerDecryptedContact = await decryptData(secrets.value.privateKey, props.tradeInfo.trade.buyer_contact!)
    sellerContact = await encryptData(props.tradeInfo.trade.arbitrator_encryption_key, userDecryptedContact)
    buyerContact = await encryptData(props.tradeInfo.trade.arbitrator_encryption_key, buyerDecryptedContact)
  }
  await client.openDispute(id, buyerContact, sellerContact)
}

async function settleDispute(winner: string) {
  await client.settleDispute(props.tradeInfo.trade.id, winner)
}
</script>

<template>
  <section class="actions card">
    <!-- # If the user is buying crypto (Buyer) -->
    <div v-if="isBuyer">
      <!-- #1 step (Optional) -->
      <!-- # A Seller requested a trade with the Buyer and it should be accepted first. -->
      <TradeAction
        v-if="tradeInfo.offer.offer.offer_type === 'buy' && tradeInfo.trade.state === 'request_created'"
        message="Review the request and accept the trade"
        :buttons="[
          {
            label: 'accept trade',
            action: () => {
              acceptTradeRequest(tradeInfo.trade.id)
            },
          },
        ]"
      />
      <!-- #2 step or #1 step -->
      <!-- if #2 step: The Buyer accepted the request and needs to wait for the Seller to deposit crypto on escrow -->
      <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the crypto on escrow -->
      <TradeAction
        v-if="
          (tradeInfo.offer.offer.offer_type === 'sell' && tradeInfo.trade.state === 'request_created') ||
          tradeInfo.trade.state === 'request_accepted'
        "
        message="Waiting for the trade to be funded"
      />
      <!-- #3 step or #2 step -->
      <!-- The crypto is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_funded'"
        message="Only press the mark as paid button after you made the payment"
        :buttons="[
          {
            label: 'mark as paid',
            action: () => {
              setFiatDeposited(tradeInfo.trade.id)
            },
          },
        ]"
      />
      <!-- #4 step or #3 step -->
      <!-- After the off-chain payment, the Buyer needs to wait for the Seller to release the funds on escrow -->
      <TradeAction v-if="tradeInfo.trade.state === 'fiat_deposited'" message="Waiting for the funds to be released" />
      <!-- #5 step or #4 step -->
      <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_released'"
        icon="check"
        message="Trade finished successfully"
      />
      <!-- expired state -->
      <TradeAction v-if="tradeInfo.trade.state === TradeState.request_expired" message="This trade has expired" />
      <!-- Trade canceled -->
      <TradeAction
        v-if="['request_canceled', 'escrow_canceled', 'escrow_refunded'].includes(tradeInfo.trade.state)"
        message="This trade has been canceled"
      />
    </div>

    <!-- # If the user is selling crypto (Seller) -->
    <div v-if="isSeller">
      <!-- #1 step (Optional) -->
      <!-- # The Seller opens the trade with the Buyer and it should be accepted first. So the Seller needs to wait. -->
      <TradeAction
        v-if="tradeInfo.offer.offer.offer_type === 'buy' && tradeInfo.trade.state === 'request_created'"
        message="Waiting for the buyer to accept the trade"
      />
      <!-- #2 step or #1 step -->
      <!-- if #2 step: The Seller needs to deposit crypto on escrow to enable the Buyer to transfer the Fiat -->
      <!-- if #1 step: The Buyer requested a trade and the Seller should accept the trade by depositing the crypto on escrow -->
      <TradeAction
        v-if="
          (tradeInfo.offer.offer.offer_type === 'sell' && tradeInfo.trade.state === 'request_created') ||
          tradeInfo.trade.state === 'request_accepted'
        "
        message="Please fund the trade"
        :buttons="[
          {
            label: 'fund trade',
            action: () => {
              fundEscrow(tradeInfo)
            },
          },
        ]"
      />
      <!-- #3 step or #2 step -->
      <!-- The crypto is on the escrow, so the Buyer needs to make the off-chain payment to mark as payed on the blockchain -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_funded'"
        message="Waiting for the buyer to make the payment"
      />
      <!-- #4 step or #3 step -->
      <!-- After the off-chain payment, the Seller needs to check the off-chain payment and release the crypto on the escrow to the Buyer -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'fiat_deposited'"
        message="Only release the funds after confirming the payment"
        :buttons="[
          {
            label: 'release funds',
            action: () => {
              releaseEscrow(tradeInfo.trade.id)
            },
          },
        ]"
      />
      <!-- #5 step or #4 step -->
      <!-- The Seller released the funds on escrow, so the Buyer already received the money on his wallet -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_released'"
        icon="check"
        message="Trade finished successfully"
      />
      <!-- Expired state -->
      <template v-if="tradeInfo.trade.state === TradeState.request_expired">
        <!-- With funds -->
        <TradeAction
          v-if="lastTradeState === TradeState.escrow_funded"
          message="This trade has expired. You have funds to be claimed."
          :buttons="[
            {
              label: 'claim funds',
              action: () => {
                refundEscrow(tradeInfo.trade.id)
              },
            },
          ]"
        />
        <!-- When refunded -->
        <TradeAction
          v-else-if="lastTradeState === TradeState.escrow_refunded"
          icon="check"
          message="Funds claimed successfully"
        />
        <!-- Without funds -->
        <TradeAction v-else message="This trade has expired" />
      </template>

      <!-- Trade canceled -->
      <TradeAction v-if="tradeInfo.trade.state === 'request_canceled'" message="This trade has been canceled" />

      <!-- Trade canceled -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_canceled'"
        message="This trade has been canceled. You have funds to be claimed. "
        :buttons="[
          {
            label: 'claim funds',
            action: () => {
              refundEscrow(tradeInfo.trade.id)
            },
          },
        ]"
      />
      <!-- Trade Refunded -->
      <TradeAction
        v-if="tradeInfo.trade.state === 'escrow_refunded'"
        icon="check"
        message="Funds claimed successfully"
      />
    </div>

    <!-- Trade Disputed -->
    <template v-if="tradeInfo.trade.state === 'escrow_disputed'">
      <TradeAction
        v-if="tradeInfo.trade.arbitrator === client.userWallet.address"
        message="Carefully review the information provided by both parties before reaching a verdict"
        :buttons="[
          {
            label: 'vote maker',
            action: () => {
              settleDispute(getMaker())
            },
          },
          {
            label: 'vote taker',
            action: () => {
              settleDispute(getTaker())
            },
          },
        ]"
      />
      <TradeAction v-else message="Dispute in progress, please wait while a decision is being made." />
    </template>

    <!-- Dispute Settled -->
    <template v-if="['settled_for_taker', 'settled_for_maker'].includes(tradeInfo.trade.state)">
      <TradeAction icon="check" message="Dispute settled for" :subMessage="disputeWinner" />
    </template>
  </section>

  <section class="sub-actions">
    <div
      v-if="
        tradeInfo.trade.state === 'request_created' ||
        tradeInfo.trade.state === 'request_accepted' ||
        (tradeInfo.trade.state === 'escrow_funded' && isBuyer)
      "
      class="wrap"
    >
      <p>Please note that requesting to cancel the transaction could impact on your reputation.</p>
      <p class="btn-action" @click="cancelTradeRequest(tradeInfo.trade.id)">Request cancel</p>
    </div>
    <template v-if="tradeInfo.trade.state === 'fiat_deposited'">
      <div v-if="timeToEnableDispute.getTime() > 0" class="wrap">
        <p>
          Depending on your choice of payment method, it can take a little longer to confirm the transaction. If this
          does not happen, an option to open a dispute will be available in
          {{ formatTimer(timeToEnableDispute, '00h 00m 00s') }}.
        </p>
      </div>
      <div v-else class="wrap">
        <p>
          If you run into problems with the transaction you can request to open a dispute. Only do this if you already
          tried to contact the other trader without success.
        </p>
        <p class="btn-action" @click="openDispute(tradeInfo.trade.id)">Request dispute</p>
      </div>
    </template>
  </section>
</template>

<style lang="scss" scoped>
@import '../../style/pages';

.actions {
  margin-top: 24px;
}

.sub-actions {
  height: 64px;

  .wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    justify-content: end;
    gap: 8px;
    margin-top: 24px;
    text-align: right;
    font-size: 14px;
    color: $gray700;

    p {
      max-width: 400px;

      &:first-child {
        font-size: 12px;
      }
    }

    .btn-action {
      cursor: pointer;
      text-decoration: underline;
    }
  }
}

button {
  background-color: $gray300;
  color: $primary;
  margin-left: auto;
  margin-top: 24px;
}
</style>
