<script setup lang="ts">
import { useClientStore } from '~/stores/client'
import { Page, trackPage } from '~/analytics/analytics'
import type { Profile } from '~/types/components.interface'
import { formatDate, timeSince } from '~/shared'

const client = useClientStore()
const route = useRoute()
const maker = computed(() => (route.params.addr as string) ?? '')
const profile = ref<Profile | undefined>()
const activeOffers = computed(() => profile.value?.active_offers_count ?? 0)
const releasedTrades = computed(() => profile.value?.released_trades_count ?? 0)
const lastTrade = computed(() => {
  const timestamp = profile.value?.last_trade ?? 0
  const date = new Date(timestamp * 1000)
  return profile.value ? timeSince(date) : 'loading...'
})
const createdAt = computed(() => {
  const timestamp = profile.value?.created_at ?? 0
  const date = new Date(timestamp * 1000)
  return profile.value ? `${formatDate(date, false, false)}` : 'loading...'
})

onMounted(() => {
  trackPage(Page.maker, { maker: maker.value })
  nextTick(async () => (profile.value = await client.fetchMakerProfile(maker.value)))
})
</script>

<template>
  <section class="page">
    <section>
      <div class="wrap-table-item">
        <div class="col-1">Profile address:</div>
        <div class="col-2">{{ maker }}</div>
      </div>
      <div class="wrap-table-item">
        <div class="col-1">Active Offers:</div>
        <div class="col-2">{{ activeOffers }}</div>
      </div>
      <div class="wrap-table-item">
        <div class="col-1">Finished Trades:</div>
        <div class="col-2">{{ releasedTrades }}</div>
      </div>
      <div class="wrap-table-item">
        <div class="col-1">Last Trade:</div>
        <div class="col-2">{{ lastTrade }}</div>
      </div>
      <div class="wrap-table-item">
        <div class="col-1">Joined:</div>
        <div class="col-2">{{ createdAt }}</div>
      </div>
    </section>
    <ListMakerOffers :maker="maker" />
  </section>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

.wrap-table-item {
  display: flex;
  flex-direction: row;
  padding: 0 16px 0 16px;
  color: $gray900;

  p {
    font-size: 14px;
    font-weight: $regular;
  }

  .col-1,
  :deep(.col-1) {
    width: 20%;
  }

  .col-2,
  :deep(.col-2) {
    width: 50%;
  }
}
</style>
