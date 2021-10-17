<template>
  <CreateOffer />
</template>

<script>
import { defineComponent } from "vue";
import CreateOffer from "@/components/CreateOffer.vue";

export default defineComponent({
  name: "Offers",
  components: {
    CreateOffer,
  },
});
</script>
