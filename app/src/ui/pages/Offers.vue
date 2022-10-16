<script setup type="ts">
const modalActive = ref(false)
function toggleModal() {
  modalActive.value = !modalActive.value
}
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
        <div class="modal-content">
          <CreateOffer @cancel="toggleModal()" />
        </div>
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
  z-index: 150;
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

.modal-content {
  display: inline-flex;
  background-color: $gray150;
  margin-top: 10%;
  z-index: 100;

  @media only screen and (max-width: $mobile) {
    margin-top: 0;
  }
  @media only screen and (max-height: 900px) {
    margin-top: 3%;
  }
}
</style>
