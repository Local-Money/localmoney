<script setup lang="ts">
const emit = defineEmits<{
  (e: 'cancel'): void
}>()
</script>

<template>
  <div class="card wrap">
    <div class="wrap-img">
      <img src="../../assets/beta-test-pop-up.png" alt="" />
    </div>

    <div class="wrap-content">
      <div class="wrap-inner">
        <h2>Welcome to our beta test!</h2>
        <h3>We are thrilled to have you join us on this exciting journey of perfecting Local Money before release.</h3>
        <p>
          Just a friendly reminder that as a beta version some features may not work as expected. Your feedback will be
          crucial in shaping the final experience to be exactly what you need. Reach out to us if you have any questions
          or run into any issues. Thank you for participating!
        </p>
      </div>
      <div class="wrap-cta">
        <button class="primary bg-gray300" @click="$emit('cancel')">Agree and continue</button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/tokens.scss';
@import '../../style/elements.scss';

.wrap {
  width: 60%;
  min-width: 700px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;

  @include responsive(mobile) {
    width: 100%;
    min-width: auto;
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
</style>
