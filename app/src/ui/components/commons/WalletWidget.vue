<script setup lang="ts"></script>

<template>
  <div class="widget">
    <div class="wrap card">
      <p class="title">My Wallet</p>
      <p class="temp-balance">$12,000 LOCAL</p>
      <button @click="$emit('disconnect')">disconnect</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.widget {
  position: absolute;
  right: 0;
  top: 64px;
}
</style>
