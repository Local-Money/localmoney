<script setup lang="ts">
const props = defineProps<{ modalActive: Boolean }>()
const emit = defineEmits<{ (e: 'close'): void }>()
</script>

<template>
  <transition name="modal-animation">
    <div v-if="modalActive" class="modal">
      <div class="modal-outside" @click="emit('close')" />
      <div class="modal-inner">
        <!-- Modal Content -->
        <slot />
      </div>
    </div>
  </transition>
</template>

<style lang="scss" scoped>
.modal-animation-enter-active {
  transition: opacity 0.3s ease;
}
.modal-animation-leave-active {
  transition: opacity 0.3s ease;
}
.modal-animation-enter-from {
  opacity: 0;
}
.modal-animation-leave-to {
  opacity: 0;
}

.modal-inner {
  display: flex;
  justify-content: center;
}

.modal-outside {
  position: fixed;
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  z-index: 150;
}
</style>
