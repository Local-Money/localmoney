<template>
  <main>
    <CreateOffer />
  </main>
</template>

<script>
import { defineComponent } from "vue";
import CreateOffer from "@/components/offers/CreateOffer.vue";

export default defineComponent({
  name: "Offers",
  components: {
    CreateOffer,
  },
});
</script>
