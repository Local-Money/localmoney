<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { usePriceStore } from '~/stores/price'
import { FiatCurrency } from '~/types/components.interface'
const price = usePriceStore()
const { getPrice } = storeToRefs(price)
const { fetchPrice } = price

onMounted(async () => {
  await fetchPrice(FiatCurrency.BRL)
})

const { t } = useI18n()

// TODO - Make isMobile global
const width = ref(window.innerWidth)
const listener = () => {
  width.value = window.innerWidth
}
onMounted(() => {
  window.addEventListener('resize', listener)
})
onUnmounted(() => {
  window.removeEventListener('resize', listener)
})
const isMobile = computed(() => width.value <= 550)
</script>

<template>
  <section>
    <div class="wrap-title">
      <h1 class="title">
        {{ t('intro.welcome') }}
      </h1>
      <p class="sub-title">
        <span class="text-primary">Local</span> is a decentralized P2P marketplace for the multi-chain world.
      </p>
      <div class="wrap-cta">
        <p>Want to know more about us?</p>

        <div class="wrap-btns">
          <a href="https://localmoney.io/local-litepaper.pdf" target="_blank">
            <button class="primary">Litepaper</button>
          </a>

          <a href="https://twitter.com/TeamLocalMoney" target="_blank">
            <svg
              class="social-icon"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <g clip-path="url(#clip0_1422_7695)">
                <path
                  d="M23.456 4.44292C23.0789 4.61015 22.6913 4.75128 22.2952 4.86586C22.7642 4.33554 23.1217 3.71154 23.3399 3.0287C23.3889 2.87564 23.3381 2.70811 23.2123 2.60801C23.0867 2.50783 22.9121 2.49583 22.7738 2.57781C21.9326 3.0767 21.0251 3.43523 20.0735 3.64488C19.115 2.70826 17.8131 2.17523 16.4673 2.17523C13.6264 2.17523 11.3151 4.48643 11.3151 7.32726C11.3151 7.551 11.3292 7.7735 11.3573 7.99291C7.83198 7.68339 4.55458 5.95066 2.304 3.18997C2.2238 3.09157 2.10024 3.03854 1.97373 3.04868C1.84715 3.05859 1.73342 3.12997 1.66947 3.23967C1.21301 4.02292 0.971694 4.91874 0.971694 5.8302C0.971694 7.07162 1.41492 8.24948 2.19786 9.16984C1.9598 9.08739 1.72878 8.98434 1.50829 8.86194C1.38991 8.79606 1.24545 8.79706 1.12785 8.8645C1.01017 8.93193 0.936391 9.05595 0.933294 9.19152C0.932752 9.21435 0.932752 9.23719 0.932752 9.26034C0.932752 11.1134 1.93007 12.7817 3.45484 13.691C3.32385 13.6779 3.19293 13.6589 3.06287 13.6341C2.92878 13.6084 2.79089 13.6554 2.70047 13.7577C2.60989 13.8599 2.57993 14.0024 2.62166 14.1324C3.18604 15.8945 4.63913 17.1906 6.39577 17.5857C4.93882 18.4983 3.27267 18.9762 1.52362 18.9762C1.15866 18.9762 0.791616 18.9548 0.432391 18.9123C0.253939 18.8911 0.0833066 18.9964 0.0225324 19.1661C-0.0382418 19.3359 0.0261712 19.5252 0.177991 19.6225C2.42493 21.0632 5.02305 21.8247 7.69131 21.8247C12.9368 21.8247 16.2182 19.3512 18.0472 17.2761C20.328 14.6887 21.6361 11.2639 21.6361 7.87995C21.6361 7.73859 21.6339 7.59583 21.6296 7.45353C22.5294 6.77557 23.3042 5.95508 23.9346 5.01203C24.0304 4.86881 24.02 4.67952 23.909 4.54767C23.7982 4.41575 23.6136 4.37309 23.456 4.44292Z"
                  fill="inherit"
                />
              </g>
            </svg>
          </a>

          <a href="https://t.co/P16BhQpcyc" target="_blank">
            <svg
              class="social-icon"
              width="32"
              height="32"
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M18.942 5.55597C17.6304 4.94227 16.2429 4.5061 14.816 4.25897C14.638 4.57997 14.431 5.01297 14.287 5.35597C12.7714 5.12497 11.2296 5.12497 9.71401 5.35597C9.55508 4.98116 9.3765 4.61498 9.17901 4.25897C7.75093 4.50651 6.36231 4.94371 5.05001 5.55897C2.43901 9.50497 1.73101 13.353 2.08501 17.146C3.60917 18.2968 5.32161 19.1742 7.14601 19.739C7.55592 19.1749 7.91849 18.5778 8.23001 17.954C7.63749 17.7282 7.06623 17.4501 6.52301 17.123C6.66601 17.017 6.80601 16.906 6.94101 16.792C10.232 18.331 13.807 18.331 17.059 16.792C17.196 16.906 17.336 17.017 17.477 17.123C16.936 17.449 16.363 17.729 15.767 17.955C16.078 18.5791 16.4406 19.1762 16.851 19.74C18.6769 19.1755 20.3905 18.2974 21.915 17.145C22.33 12.749 21.206 8.93597 18.942 5.55597V5.55597ZM8.67801 14.813C7.69001 14.813 6.88001 13.891 6.88001 12.768C6.88001 11.645 7.67301 10.721 8.67801 10.721C9.68301 10.721 10.493 11.643 10.476 12.768C10.477 13.891 9.68301 14.813 8.67801 14.813ZM15.322 14.813C14.334 14.813 13.524 13.891 13.524 12.768C13.524 11.645 14.317 10.721 15.322 10.721C16.327 10.721 17.137 11.643 17.12 12.768C17.12 13.891 16.327 14.813 15.322 14.813Z"
                fill="inherit"
              />
            </svg>
          </a>
        </div>
      </div>
    </div>

    <div v-if="!isMobile" class="wrap-img">
      <img src="../assets/header-img.png" />
    </div>
  </section>
</template>

<style lang="scss" scoped>
@import '../style/tokens.scss';

section {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 64px 0 24px;
}

.wrap-title {
  width: 55%;
}

.wrap-img {
  width: 45%;

  img {
    width: 100%;
    padding: 0 24px;
    opacity: 0.9;
  }
}

h1 {
  font-size: 40px;
  font-weight: 600;
  line-height: 120%;
}

.title {
  margin-bottom: 24px;
}
.sub-title {
  width: 80%;
  font-size: 22px;
  font-weight: 400;
  margin-bottom: 32px;

  .text-primary {
    color: $primary;
    font-weight: 600;
  }
}

.wrap-cta {
  .wrap-btns {
    display: flex;
    align-items: center;
    gap: 24px;
    margin-top: 24px;
  }
  a {
  }
  p {
    font-size: 16px;
  }
  .social-icon {
    fill: $base-text;
    opacity: 0.5;
    transition: 300ms;
    padding-top: 2px;
    &:hover {
      opacity: 1;
      fill: $primary;
      transition: 300ms;
    }
  }
}

@media only screen and (max-width: $mobile) {
  section {
    display: flex;
    flex-direction: column-reverse;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 24px 0 32px;

    h1 {
      font-size: 38px;
    }

    .sub-title {
      font-size: 22px;
    }

    .wrap-title {
      width: 100%;
      text-align: center;
    }

    .wrap-img {
      width: 50%;
    }

    .sub-title {
      width: 100%;
    }

    .wrap-cta {
      .wrap-btns {
        display: flex;
        align-content: center;
        justify-content: center;
      }
    }
  }
}
</style>
