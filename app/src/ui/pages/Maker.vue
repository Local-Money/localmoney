<script setup lang="ts">
import { useClientStore } from '~/stores/client'
import { ChainClient } from '~/network/Chain'
import { Page, trackPage } from '~/analytics/analytics'
import type { Profile } from '~/types/components.interface'
import { formatAddress, formatDate, timeSince } from '~/shared'

const client = useClientStore()
const route = useRoute()
const maker = computed(() => (route.params.addr as string) ?? '')
const makerAddr = computed(() => formatAddress(maker.value))
const profile = ref<Profile | undefined>()
const activeOffers = computed(() => profile.value?.active_offers_count ?? 0)
const releasedTrades = computed(() => profile.value?.released_trades_count ?? 0)
const lastTrade = computed(() => {
  const timestamp = profile.value?.last_trade ?? 0
  const date = new Date(timestamp * 1000)
  return profile.value ? timeSince(date) : 'loading...'
})
const createdAt = computed(() => {
  const timestamp = profile.value?.created_at ?? 0
  const date = new Date(timestamp * 1000)
  return profile.value ? `${formatDate(date, false, false)}` : 'loading...'
})
const finder = computed(() => {
  if (client.chainClient === ChainClient.kujiraMainnet) {
    return `kaiyo-1/address/${maker.value}`
  } else {
    return `harpoon-4/address/${maker.value}`
  }
})

onMounted(() => {
  trackPage(Page.maker, { maker: maker.value })
  nextTick(async () => (profile.value = await client.fetchMakerProfile(maker.value)))
})
</script>

<template>
  <section class="page">
    <div class="header">
      <div class="wrap-profile">
        <div class="inner-wrap">
          <div class="maker">
            <h2 class="maker-addr">{{ makerAddr }}</h2>
            <div class="actions">
              <a :href="`https://finder.kujira.app/${finder}`" target="_blank" alt="Finder link">
                <svg
                  class="icon-24"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M18 13V19C18 19.5304 17.7893 20.0391 17.4142 20.4142C17.0391 20.7893 16.5304 21 16 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V8C3 7.46957 3.21071 6.96086 3.58579 6.58579C3.96086 6.21071 4.46957 6 5 6H11"
                    stroke="inherit"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M15 3H21V9"
                    stroke="inherit"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M10 14L21 3"
                    stroke="inherit"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
              </a>
            </div>
          </div>
          <div class="info">
            <p class="trade-completed">Completed trades: {{ releasedTrades }}</p>
            <p class="last-seen">Last trade: {{ lastTrade }}</p>
            <p class="joined">Joined {{ createdAt }}</p>
          </div>
        </div>
      </div>
    </div>

    <ListMakerOffers :maker="maker" />
  </section>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

.header {
  height: 150px;
  display: flex;
  align-items: center;
  border-bottom: 1px solid $gray200;
  margin-bottom: 56px;

  .wrap-profile {
    display: flex;
    gap: 24px;
    .pfp {
      .img {
        width: 56px;
        height: 56px;
        background-color: $surface;
        border-radius: 200px;
      }
    }
    .inner-wrap {
      .maker {
        display: flex;
        gap: 16px;

        .maker-addr {
          font-size: 24px;
          font-weight: $bold;
          margin-bottom: 4px;
        }
        .actions {
          display: flex;
          a {
            margin-top: 7px;
          }
          svg {
            width: 20px;
            height: 20px;
            stroke: $gray600;
          }
        }
      }
      .info {
        display: flex;
        gap: 12px;

        .trade-completed,
        .last-seen,
        .joined {
          font-size: 14px;
          font-weight: $regular;
          color: $gray700;
          border-right: 1px solid $border;
          padding-right: 12px;

          &:last-child {
            border: none;
            padding: 0;
          }
        }
      }
    }
  }
}

.wrap-table-item {
  display: flex;
  flex-direction: row;
  padding: 0 16px 0 16px;
  color: $gray900;

  p {
    font-size: 14px;
    font-weight: $regular;
  }

  .col-1,
  :deep(.col-1) {
    width: 20%;
  }

  .col-2,
  :deep(.col-2) {
    width: 50%;
  }
}
</style>
