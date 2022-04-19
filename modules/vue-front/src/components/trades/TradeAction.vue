<template>
  <div class="wrap">
    <div class="icon">
      <RightArrow/>
    </div>
    <p>{{ message }}</p>
    <button v-if="buttonLabel" @click="$emit('actionClick')">
      {{ buttonLabel }}
    </button>
  </div>
</template>
<script>
import RightArrow from "@/components/commons/RightArrow";
import { defineComponent } from "vue";

export default defineComponent({
  name: 'TradeAction',
  props: {
    message: {
      type: String,
      required: true
    },
    buttonLabel: {
      type: String,
      required: false
    }
  },
  components: { RightArrow }
});
</script>
<style lang="scss" scoped>
@import "../../style/pages";

.wrap {
  display: flex;
  align-items: center;

  .icon {
    stroke: $primary;
  }

  p {
    width: 50%;
    font-size: 16px;
    font-weight: 700;
    margin-left: 24px;
  }
}

button {
  background-color: $gray300;
  color: $primary;
  margin-left: auto;
}
</style>