<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useClientStore } from '~/stores/client'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
onMounted(() => {
  nextTick(async () => await client.fetchMyTrades())
})

onUnmounted(async () => {})

watch(userWallet, () => {
  nextTick(async () => await client.fetchMyTrades())
})

const proposalId = ref('')
</script>

<template>
  <main class="page">
    <input v-model="proposalId" type="text" style="color: black" />
    <button
      class="btn btn-primary"
      @click="
        client.voteProposal(
          'kujira15nu6cdlfvwyshj5fnwt09ppx4e6z9gddtkpxfrr83femj67dunhs84rh3j',
          parseInt(proposalId),
          'yes'
        )
      "
    >
      Yes
    </button>
  </main>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

main {
  margin-bottom: 48px;
}
.wrap-title {
  display: flex;
}

h3 {
  margin: 32px 0;
  font-size: 18px;
  font-weight: $semi-bold;
}

.trade-history-table {
  overflow: auto;
  .table-header {
    display: flex;
    flex-direction: row;
    border-bottom: 1px solid $border;
    padding: 16px;
    margin-bottom: 16px;

    p {
      font-size: 14px;
      font-weight: $semi-bold;
      color: $gray700;
    }

    @media only screen and (max-width: $mobile) {
      min-width: 1000px;
      padding: 0 0 16px 0;
    }
  }
}

.col-1,
:deep(.col-1) {
  width: 10%;
}

.col-2,
:deep(.col-2) {
  width: 20%;
}

.col-3,
:deep(.col-3) {
  width: 7.5%;
}

.col-4,
:deep(.col-4) {
  width: 17%;
}

.col-5,
:deep(.col-5) {
  width: 17%;
}

.col-6,
:deep(.col-6) {
  width: 20%;
}

.col-7,
:deep(.col-7) {
  width: 10%;
}
</style>
