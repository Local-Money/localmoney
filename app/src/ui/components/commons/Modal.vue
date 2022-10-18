<script setup lang="ts">
const props = defineProps<{ modalActive: Boolean }>()
const emit = defineEmits<{ (e: 'close'): void }>()
</script>

<template>
  <transition name="modal-animation">
    <div v-if="modalActive" class="modal">
      <div class="modal-inner">
        <!-- Modal Content -->
        <slot />
      </div>
      <div class="modal-overlay" @click="emit('close')" />
    </div>
  </transition>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';
@import '../../style/elements.scss';
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

.modal {
  display: inline-flex;
  justify-content: center;
}

.modal-inner {
  height: 100px;
  z-index: $z-modal-content;
  margin-top: 100px;

  @media only screen and (max-width: $mobile) {
    margin-top: 0;
  }
  @media only screen and (max-height: 900px) {
    margin-top: 3%;
  }
}

.modal-overlay {
  position: fixed;
  width: 100%;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  z-index: $z-modal-overlay;
}
</style>
