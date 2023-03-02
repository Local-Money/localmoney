<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useClientStore } from '~/stores/client'
import { enableDisputes, enableMyOffers } from '~/config/featureToggle'

const client = useClientStore()
const { userWallet } = storeToRefs(client)
const enableDisputesNav = computed(() => enableDisputes(userWallet.value, client.arbitrators.data))
const enableMyOffersNav = computed(() => enableMyOffers(userWallet.value))
</script>

<template>
  <nav>
    <ul>
      <li v-if="enableMyOffersNav" class="item">
        <router-link to="/offers">
          <svg class="icon-24" viewBox="0 0 24 24" fill="none">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
            <path d="M2 17L12 22L22 17" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
            <path d="M2 12L12 17L22 12" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <p>My Offers</p>
        </router-link>
      </li>

      <li class="item">
        <router-link to="/trades">
          <svg class="icon-24" viewBox="0 0 24 24" fill="none">
            <path
              d="M2 3H8C9.06087 3 10.0783 3.42143 10.8284 4.17157C11.5786 4.92172 12 5.93913 12 7V21C12 20.2044 11.6839 19.4413 11.1213 18.8787C10.5587 18.3161 9.79565 18 9 18H2V3Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M22 3H16C14.9391 3 13.9217 3.42143 13.1716 4.17157C12.4214 4.92172 12 5.93913 12 7V21C12 20.2044 12.3161 19.4413 12.8787 18.8787C13.4413 18.3161 14.2044 18 15 18H22V3Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          <p>My Trades</p>
        </router-link>
      </li>

      <li v-if="enableDisputesNav" class="item">
        <router-link to="/arbitration">
          <svg
            class="icon-24"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M21 15C21 15.5304 20.7893 16.0391 20.4142 16.4142C20.0391 16.7893 19.5304 17 19 17H7L3 21V5C3 4.46957 3.21071 3.96086 3.58579 3.58579C3.96086 3.21071 4.46957 3 5 3H19C19.5304 3 20.0391 3.21071 20.4142 3.58579C20.7893 3.96086 21 4.46957 21 5V15Z"
              stroke="inherit"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          <p>Disputes</p>
        </router-link>
      </li>

      <li class="item">
        <NotificationWidget />
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
@import '../style/tokens.scss';

nav {
  display: flex;
  justify-content: space-between;
  margin-left: auto;

  ul {
    display: flex;

    li {
      padding: 0 20px;

      @media only screen and (max-width: 1080px) {
        padding: 0 10px;
      }
    }

    a {
      display: inline-block;
      height: 40px;
      font-size: 14px;
      color: $base-text;

      &.router-link-active {
        .icon-24 {
          stroke: $primary;
        }
      }

      &:hover {
        color: $gray900;
        .icon-24 {
          stroke: $primary;
        }
      }

      .icon-24 {
        vertical-align: middle;
      }

      @media only screen and (max-width: 1150px) {
        svg {
          display: none;
        }
      }

      p {
        display: inline-block;
        vertical-align: middle;
        line-height: 35px;
        margin-left: 16px;
      }
    }
  }
}
</style>
