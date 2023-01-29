<script setup lang="ts">
const props = defineProps<{
  show: boolean
}>()
const emit = defineEmits<{
  (e: 'accept'): void
}>()
</script>

<template>
  <transition name="modal-animation">
    <div v-if="props.show" class="modal">
      <div class="card wrap">
        <div class="wrap-img">
          <img src="../../assets/beta-test-pop-up.png" alt="" />
        </div>

        <div class="wrap-content">
          <div class="wrap-inner">
            <h2>Welcome to our beta test!</h2>
            <h3></h3>
            <p>
              We appreciate you joining us for this test. Your input will be crucial in making the final product even
              better. If you encounter any issues or have any questions, please don't hesitate to contact us.
            </p>
          </div>
          <div class="wrap-cta">
            <button class="primary bg-gray300" @click="emit('accept')">Agree and continue</button>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';
@import '../../style/elements.scss';

.wrap {
  width: 60%;
  min-width: 700px;
  max-width: 1000px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;

  @include responsive(mobile) {
    width: 100%;
    min-width: auto;
  }

  @media only screen and (max-height: 700px) {
    overflow-y: scroll;
  }

  .wrap-img {
    width: 100%;
    margin-top: 40px;
    margin-bottom: 16px;
    img {
      width: 100%;
    }
  }

  .wrap-content {
    width: 80%;
    display: flex;
    text-align: center;
    flex-direction: column;
    gap: 40px;

    @include responsive(mobile) {
      width: 90%;
    }

    .wrap-inner {
      margin-bottom: 16px;
      h2 {
        font-size: 32px;
        margin-bottom: 16px;
      }
      p {
      }
    }

    .wrap-cta {
      display: flex;
      justify-content: space-around;
      margin-bottom: 40px;
    }
  }
}

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
  z-index: $z-modal-loading;
  backdrop-filter: blur(10px);

  @media only screen and (max-height: 800px) {
    align-items: unset;
  }
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
