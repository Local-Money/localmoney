<template>
  <main>
    <HomeHero />
    <Offers />
  </main>
</template>

<script>
import { defineComponent } from 'vue'
import HomeHero from './HomeHero.vue'
import Offers from './Offers.vue'

export default defineComponent({
  name: 'Home',
  components: {
    HomeHero,
    Offers,
  },
})
</script>

<style lang="scss" scoped>
@import '../style/header.scss';
</style>
