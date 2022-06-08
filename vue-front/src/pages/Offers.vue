<template>
    <main>
        <div class="wrap-title">
            <div class="inner-wrap">
                <h3>My Offers</h3>
                <div class="btn-add" @click="toggleModal">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
                        <path
                            d="M12 5V19"
                            stroke="inherit"
                            stroke-width="2"
                            stroke-linecap="round"
                        />
                        <path
                            d="M5 12L19 12"
                            stroke="inherit"
                            stroke-width="2"
                            stroke-linecap="round"
                        />
                    </svg>
                </div>
            </div>

            <!-- <button @click="toggleModal">Create a new offer</button> -->

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
