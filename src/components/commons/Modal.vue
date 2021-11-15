<template>
  <transition name="modal-animation">
    <div @click="$emit('close')" v-if="modalActive" class="modal-overlay">
      <div v-if="modalActive" class="modal-inner">
        <!-- Modal Content -->
        <slot />
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  props: ["modalActive"],
};
</script>

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
</style>
