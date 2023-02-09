<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useRouter } from 'vue-router'
import { useClientStore } from '~/stores/client'
import { enableDisputes } from '~/config/featureToggle'
import { Page, trackPage } from '~/analytics/analytics'

const client = useClientStore()
const router = useRouter()
const { userWallet } = storeToRefs(client)
const enableDisputesPage = computed(() => enableDisputes(userWallet.value, client.arbitrators.data))

onBeforeMount(() => {
  if (!enableDisputesPage.value) {
    router.push('/')
  }
})

onMounted(() => {
  trackPage(Page.disputes)
})
</script>

<template>
  <main class="page">
    <div class="wrap-title">
      <h3>Open Disputes</h3>
    </div>
    <ListDisputes />
  </main>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

.wrap-title {
  display: flex;
}

h3 {
  margin: 32px 0;
  font-size: 18px;
  font-weight: $semi-bold;
}
</style>
