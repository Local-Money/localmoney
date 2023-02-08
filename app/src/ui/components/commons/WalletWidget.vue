<script setup lang="ts">
// import { useClientStore } from '~/stores/client'
// import { microDenomToDenom } from '~/utils/denom'
// import { formatAmount } from '~/shared'
// const client = useClientStore()
// const acceptedBalance = computed(() => client.acceptedTokenBalance)
// const receivedBalance = computed(() => client.receivedTokenBalance)

// function formatDenom(denom: string): string {
//   return microDenomToDenom(denom)
// }

const widgetActive = ref(false)
function toggleWidget() {
  widgetActive.value = !widgetActive.value
}
defineExpose({ toggleWidget })
</script>

<template>
  <div v-if="widgetActive" class="widget">
    <div class="wrap card">
      <p class="title">My Wallet</p>
      <!-- <div class="balance">
        <p class="label">{{ formatDenom(receivedBalance.denom) }}</p>
        <p class="balance">{{ formatAmount(receivedBalance.amount) }}</p>
      </div> -->
      <div class="balance">
        <div class="token">
          <img src="../../assets/logo-icon-dark.svg" alt="" />
          <p class="label">LOCAL</p>
        </div>

        <p class="balance">$34,000.854</p>
      </div>
      <button class="secondary bg-gray300 center-text" @click="$emit('connectWidget')">disconnect</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.widget {
  position: absolute;
  right: 0;
  top: 64px;

  .wrap {
    min-width: 350px;
    display: flex;
    flex-direction: column;
    gap: 24px;

    .title {
      padding-bottom: 16px;
      margin-bottom: 8px;
      border-bottom: 1px solid $border;
    }

    .balance {
      display: flex;
      justify-content: space-between;

      .token {
        display: flex;
        gap: 8px;
      }
    }

    button {
      width: 100%;
    }
  }
}
</style>
