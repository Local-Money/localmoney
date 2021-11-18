<template>
  <transition name="modal-animation">
    <div v-if="loading.isLoading" class="modal">
      <div class="loading">
        <div class="spinner">
          <svg
            viewBox="0 0 100 100"
            enable-background="new 0 0 0 0"
            xml:space="preserve"
          >
            <path
              d="M73,50c0-12.7-10.3-23-23-23S27,37.3,27,50 M30.9,50c0-10.5,8.5-19.1,19.1-19.1S69.1,39.5,69.1,50"
            >
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
        <div class="loading-content">
          <p class="label">{{ loading.label }}</p>
          <a class="transaction" :href="'#' + loading.transaction">{{
            formatAddress(loading.transaction)
          }}</a>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
import { formatAddress } from "@/shared";

export default {
  props: ["loading"],
  methods: {
    formatAddress,
  },
};
</script>

<style lang="scss" scoped>
@import "@/style/tokens.scss";

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
  position: fixed;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  backdrop-filter: blur(10px);
}

.loading {
  display: flex;
  flex-direction: column;
  text-align: center;

  .spinner {
    svg {
      width: 56px;
      height: 56px;
      fill: $primary;
    }
  }

  .loading-content {
    .label {
      font-size: 16px;
    }
    .transaction {
      display: block;
      font-size: 14px;
      font-weight: 600;
      color: $primary;
      margin-top: 8px;
    }
  }
}
</style>
