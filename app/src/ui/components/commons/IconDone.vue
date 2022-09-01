<script setup lang="ts"></script>

<template>
  <div class="icon done">
    <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M20 6L9 17L4 12" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens';

.done {
  svg {
    stroke: $primary;
  }
}
</style>
