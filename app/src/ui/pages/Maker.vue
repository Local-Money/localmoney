<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useClientStore } from '~/stores/client'
import { Page, trackPage } from '~/analytics/analytics'

const client = useClientStore()
const router = useRouter()

onBeforeMount(() => {})

onMounted(() => {
  trackPage(Page.maker)
})
</script>

<template>
  <section class="page">
    <ListMakerOffers />
  </section>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';
</style>
