<script setup lang="ts">
import type { TradeInfo } from '~/types/components.interface'
import { useClientStore } from '~/stores/client'
import { decryptData, encryptData } from '~/utils/crypto'

const props = defineProps<{
  tradeInfo: TradeInfo
  walletAddress: String
}>()

const buyerOrSeller = computed(() => (isBuyer.value ? 'Buyer' : 'Seller'))
const counterpartyEncryptedContact = computed(() =>
  isBuyer.value ? tradeInfo.value.trade.seller_contact : tradeInfo.value.trade.buyer_contact
)
const isCounterpartyContactAvailable = computed(() => counterpartyEncryptedContact.value !== null)
const counterpartyContact = asyncComputed(async () => {
  const encryptedContact = counterpartyEncryptedContact.value
  const privateKey = secrets.value.privateKey
  if (isCounterpartyContactAvailable.value) {
    console.log('decrypt')
    return await decryptData(privateKey, encryptedContact)
  } else {
    return 'pending ...'
  }
})
</script>

<template>
  <div class="chat card">
    <p>Contact Information</p>

    <div class="content">
      <p class="label">
        {{ buyerOrSeller }}:
        <a
          v-if="isCounterpartyContactAvailable"
          :href="addTelegramURLPrefix(counterpartyContact)"
          class="telegram"
          target="_blank"
        >
          {{ counterpartyContact }}
        </a>
        <span v-else class="label">Pending ...</span>
      </p>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/pages';
</style>
