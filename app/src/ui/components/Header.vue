<script setup lang="ts">
import NavDesktop from './NavDesktop.vue'
import NavMobile from './NavMobile.vue'
import { useClientStore } from '~/stores/client'
import { ChainClient } from '~/network/Chain'

const client = useClientStore()
const badge = computed(() => (client.chainClient === ChainClient.kujiraMainnet ? 'Mainnet' : 'Testnet'))
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
  <header>
    <div class="wrap">
      <div class="wrap-logo">
        <router-link :to="{ path: '/' }">
          <div className="logo" />
        </router-link>

        <div class="badge-testnet">
          <p>{{ badge }}</p>
        </div>
      </div>

      <NavMobile v-if="isMobile" />
      <NavDesktop v-else />
      <WalletButton />
    </div>
  </header>
</template>

<style lang="scss" scoped>
@import '../style/tokens.scss';

header {
  position: fixed;
  height: 100px;
  top: 0;
  background-color: $background;
  width: 100%;
  min-width: 1000px;
  z-index: $z-navigation;

  @media only screen and (max-width: $mobile) {
    position: relative;
    min-width: 0;
    height: auto;
  }

  .wrap {
    display: flex;
    justify-content: space-between;
    align-content: center;
    margin: 16px auto 0;
    max-width: 1400px;
    padding: 16px 32px;

    .wrap-logo {
      display: flex;
      align-content: center;

      .logo {
        width: 220px;
        height: 42px;
        background-size: cover;
        background-image: $logo-horizontal-dark;
      }

      .badge-testnet {
        display: flex;
        margin-left: 24px;
        align-items: center;

        p {
          font-size: 13px;
          color: $primary;
          font-weight: 400;
          background-color: $surface;
          border-radius: 8px;
          padding: 6px 14px;
        }
      }
    }

    @media only screen and (max-width: 550px) {
      margin: 0 auto;
      padding: 0px;

      .wrap-logo {
        padding: 16px 0 0 16px;

        a {
          display: flex;
          align-items: center;
        }

        .logo {
          width: 32px;
          height: 32px;
          margin-top: 0px;
          margin-left: 0px;
          background-size: cover;
          background-image: $logo-icon-dark;
        }

        .badge-testnet {
          margin-left: 16px;
        }
      }
    }
  }
}
</style>
