<template>
    <main>
        <div class="wrap-title">
            <h3>My Offers</h3>
            <button @click="toggleModal">Create a new offer</button>

            <Modal @close="toggleModal" :modalActive="modalActive">
                <div class="modal-content card">
                    <CreateOffer @cancel="toggleModal" />
                </div>
            </Modal>
        </div>
        <ListMyOffers />
    </main>
</template>

<script>
import { defineComponent } from "vue";
import CreateOffer from "@/components/offers/CreateOffer.vue";
import ListMyOffers from "@/components/myOffers/ListMyOffers.vue";
import Modal from "@/components/commons/Modal.vue";
import { ref } from "vue";

export default defineComponent({
    name: "Offers",
    components: {
        CreateOffer,
        ListMyOffers,
        Modal,
    },
    setup() {
        const modalActive = ref(false);
        const toggleModal = () => {
            modalActive.value = !modalActive.value;
        };
        return { modalActive, toggleModal, ListMyOffers };
    },
});
</script>

<style lang="scss" scoped>
@import "@/style/pages.scss";

.wrap-title {
    display: flex;
    justify-content: space-between;
}

button {
    margin-top: 32px;
    background-color: $surface;
    color: $primary;
}

.modal {
    position: fixed;
    width: 100%;
    height: 100vh;
    left: 0;
    top: 0;
    backdrop-filter: blur(10px);
}

.modal-content {
    display: inline-flex;
    background-color: $gray150;
    margin-top: 10%;
    z-index: 100;
}
</style>
