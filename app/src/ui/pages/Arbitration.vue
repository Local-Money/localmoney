<script setup lang="ts"></script>

<template>
  <main class="page">
    <div class="wrap-title">
      <h3>Open Disputes</h3>
    </div>
    <ListDisputes />
  </main>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

.wrap-title {
  display: flex;
}

h3 {
  margin: 32px 0;
  font-size: 18px;
  font-weight: $semi-bold;
}
</style>
