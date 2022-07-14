<script setup lang="ts">
const props = defineProps<{
  message: string
  buttonLabel?: string
}>()

const emit = defineEmits<{
  (e: 'actionClick'): void
}>()

function click() {
  emit('actionClick')
}
</script>

<template>
  <div class="wrap">
    <div class="icon">
      <RightArrow />
    </div>
    <p>{{ message }}</p>
    <button v-if="buttonLabel" class="primary" @click="click()">
      {{ buttonLabel }}
    </button>
  </div>
</template>

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
  margin-left: auto;
}
</style>
