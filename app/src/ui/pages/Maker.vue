<script setup lang="ts">
import { useClientStore } from '~/stores/client'
import { Page, trackPage } from '~/analytics/analytics'

const client = useClientStore()
const route = useRoute()
const maker = computed(() => (route.params.addr as string) ?? '')

onMounted(() => {
  trackPage(Page.maker, { maker: maker.value })
})
</script>

<template>
  <section class="page">
    <ListMakerOffers :maker="maker" />
  </section>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';
</style>
