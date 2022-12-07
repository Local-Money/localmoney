<script setup lang="ts">
defineProps<{ icon: string }>()
</script>

<template>
  <div :class="$attrs.class">
    <template v-if="icon === 'arrow'">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M5 12H19" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M12 5L19 12L12 19" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </template>
    <template v-if="icon === 'check'">
      <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M20 6L9 17L4 12" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </template>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens';

.color {
  svg {
    stroke: $primary;
  }
}
.done {
  svg {
    stroke: $done;
  }
}
</style>
