<template>
  <header>
    <div class="wrap">
      <router-link :to="{ path: '/' }">
        <div className="logo"></div>
      </router-link>

      <nav-mobile v-if="isMobile()"> </nav-mobile>
      <nav-desktop v-else> </nav-desktop>

      <button class="wallet" @click="initWallet()">
        <p v-if="walletAddress.length > 0">
          {{ formatAddress(walletAddress) }}
        </p>
        <p v-if="walletAddress.length === 0">connect</p>
        <img src="@/assets/ic_wallet.svg" alt="Connect your wallet" />
      </button>
    </div>
  </header>
</template>

<script>
import { defineComponent } from "vue";
import { formatAddress, formatAmount } from "@/shared";
import { mapActions, mapGetters } from "vuex";
import NavDesktop from "./NavDesktop.vue";
import NavMobile from "./NavMobile.vue";

export default defineComponent({
  name: "Header",
  components: {
    NavDesktop,
    NavMobile,
  },
  methods: {
    ...mapActions(["initWallet"]),
    formatAmount,
    formatAddress,
    isMobile() {
      if (screen.width <= 760) {
        return true;
      } else {
        return false;
      }
    },
  },
  computed: mapGetters(["walletAddress"]),
});
</script>

<style lang="scss" scoped>
@import "../style/tokens.scss";

header {
  background-color: $background;
  width: 100%;

  .wrap {
    display: flex;
    justify-content: space-between;
    align-content: center;
    margin: 0 auto;
    max-width: 1900px;
    padding: 16px 32px;

    .logo {
      width: 191px;
      height: 40px;
      background-size: cover;
      background-image: $logo-horizontal-dark;
    }

    @media only screen and (max-width: 550px) {
      padding: 0;
      .logo {
        width: 32px;
        height: 32px;
        margin-top: 24px;
        margin-left: 24px;
        background-size: cover;
        background-image: $logo-horizontal-dark;
      }
    }
  }
}
</style>
