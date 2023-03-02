<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useRouter } from 'vue-router'
import { useClientStore } from '~/stores/client'
import { enableMyOffers } from '~/config/featureToggle'
import { Page, trackPage } from '~/analytics/analytics'

const client = useClientStore()
const router = useRouter()
const { userWallet } = storeToRefs(client)
const enableMyOffersPage = computed(() => enableMyOffers(userWallet.value))

const modalActive = ref(false)
function toggleModal() {
  modalActive.value = !modalActive.value
}

onBeforeMount(() => {
  if (!enableMyOffersPage.value) {
    router.push('/')
  }
})

onMounted(() => {
  trackPage(Page.my_offers)
})
</script>

<template>
  <section class="page">
    <div class="wrap-title">
      <div class="inner-wrap">
        <h3>My Offers</h3>
        <div class="btn-add" @click="toggleModal">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
            <path d="M12 5V19" stroke="inherit" stroke-width="2" stroke-linecap="round" />
            <path d="M5 12L19 12" stroke="inherit" stroke-width="2" stroke-linecap="round" />
          </svg>
        </div>
      </div>

      <Modal :modalActive="modalActive" @close="toggleModal()">
        <CreateOffer @cancel="toggleModal()" />
      </Modal>
    </div>
    <ListMyOffers />
  </section>
</template>

<style lang="scss" scoped>
@import '../style/pages.scss';

.wrap-title {
  display: flex;
  justify-content: space-between;

  .inner-wrap {
    display: flex;
    align-items: center;

    .btn-add {
      width: 40px;
      height: 40px;
      background-color: $surface;
      border-radius: 8px;
      margin-left: 24px;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;

      svg {
        stroke: $primary;
        vertical-align: middle;
      }
    }
  }
}

button {
  margin-top: 32px;
  background-color: $surface;
  color: $primary;
}

.modal {
  position: fixed;
  width: 100%;
  left: 0;
  top: 0;
  bottom: 0;
  z-index: $z-modal-overlay;
  backdrop-filter: blur(10px);

  overflow-y: auto;
  &::-webkit-scrollbar {
    width: 8px;
    background: $gray100;
  }
  &::-webkit-scrollbar-thumb {
    background: $gray300;
    border-radius: 56px;
  }
}
</style>
