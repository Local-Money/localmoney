<script setup lang="ts">
interface TradeActionButton {
  label: string
  action: () => void
}

const props = defineProps<{
  message: string
  buttons?: TradeActionButton[]
}>()
</script>

<template>
  <div class="wrap">
    <div class="icon">
      <RightArrow />
    </div>
    <p>{{ message }}</p>
    <button v-for="button in props.buttons" :key="button.label" class="primary" @click="button.action()">
      {{ button.label }}
    </button>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/pages';

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
  margin-left: auto;
}
</style>
