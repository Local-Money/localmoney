<script setup lang="ts"></script>

<template>
  <div class="spinner">
    <svg viewBox="0 0 100 100" enable-background="new 0 0 0 0" xml:space="preserve">
      <path d="M73,50c0-12.7-10.3-23-23-23S27,37.3,27,50 M30.9,50c0-10.5,8.5-19.1,19.1-19.1S69.1,39.5,69.1,50">
        <animateTransform
          attributeName="transform"
          type="rotate"
          dur="0.7s"
          from="0 50 50"
          to="360 50 50"
          repeatCount="indefinite"
        />
      </path>
    </svg>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';

.spinner {
  svg {
    width: 56px;
    height: 56px;
    fill: $primary;
  }
}
</style>
