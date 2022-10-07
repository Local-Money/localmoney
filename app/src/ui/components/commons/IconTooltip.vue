<script setup lang="ts">
defineProps(['content'])
</script>

<template>
  <div class="tooltip">
    <svg class="icon-24" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
        stroke="inherit"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <path
        d="M9.09 9.00002C9.3251 8.33169 9.78915 7.76813 10.4 7.40915C11.0108 7.05018 11.7289 6.91896 12.4272 7.03873C13.1255 7.15851 13.7588 7.52154 14.2151 8.06355C14.6713 8.60555 14.9211 9.29154 14.92 10C14.92 12 11.92 13 11.92 13"
        stroke="inherit"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
      <path d="M12 17H12.01" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
    <p class="tooltip-content">
      {{ content }}
    </p>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens';

.tooltip {
  position: relative;
  height: 20px;
  cursor: pointer;

  &:hover {
    svg {
      stroke: $primary;
    }
  }

  svg {
    width: 20px;
    height: 20px;
    stroke: $gray700;
  }

  .tooltip-content {
    position: absolute;
    z-index: 100;
    visibility: hidden;
    width: 250px;
    bottom: 180%;
    left: 50%;
    transform: translateX(-50%);
    background-color: $gray100;
    font-size: 11px;
    font-weight: $regular;
    color: $base-text;
    text-align: center;
    padding: 8px;
    border-radius: 8px;
    opacity: 0;
    transition: all 0.15s ease-in-out;

    &:after {
      content: ' ';
      position: absolute;
      top: 100%; /* At the bottom of the tooltip */
      left: 50%;
      margin-left: -5px;
      border-width: 5px;
      border-style: solid;
      border-color: $gray100 transparent transparent transparent;
    }
  }
  &:hover .tooltip-content {
    visibility: visible;
    opacity: 1;
  }
}
</style>
