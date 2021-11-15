<template>
  <main>
    <h3>My Offers</h3>
    <div class="card offer-list">
      <p>Nothing here for now.</p>
    </div>
    <p>You can create your own offers and earn fees.</p>
    <button @click="toggleModal">Create a new offer</button>

    <Modal @close="toggleModal" :modalActive="modalActive">
      <div class="modal-content card">
        <CreateOffer @cancel="toggleModal" />
      </div>
    </Modal>
  </main>
</template>

<script>
import { defineComponent } from "vue";
import CreateOffer from "@/components/offers/CreateOffer.vue";
import Modal from "@/components/commons/Modal.vue";
import { ref } from "vue";

export default defineComponent({
  name: "Offers",
  components: {
    CreateOffer,
    Modal,
  },
  setup() {
    const modalActive = ref(false);
    const toggleModal = () => {
      modalActive.value = !modalActive.value;
    };
    return { modalActive, toggleModal };
  },
});
</script>

<style lang="scss" scoped>
@import "@/style/pages.scss";

.offer-list {
  margin-bottom: 32px;
}

button {
  margin-top: 32px;
  background-color: $surface;
  color: $primary;
}

.modal-overlay {
  position: fixed;
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  //background-color: rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
}

.modal-content {
  display: inline-flex;
  background-color: $gray150;
  margin-top: 10%;
}
</style>
