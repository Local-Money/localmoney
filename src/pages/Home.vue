<template>
  <main>
    <HomeHero />
    <ListOffers />
  </main>
</template>

<script>
import { defineComponent } from "vue";
import HomeHero from "../components/HomeHero.vue";
import ListOffers from "../components/ListOffers.vue";

export default defineComponent({
  name: "Home",
  components: {
    HomeHero,
    ListOffers,
  },
});
</script>

<style lang="scss" scoped>
@import "../style/header.scss";
</style>
