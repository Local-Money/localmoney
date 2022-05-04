<template>
    <Header />
    <router-view />

    <!-- Loading Modal -->
    <ModalLoading :loading="loading" />
</template>

<script>
import { defineComponent } from "vue";
import Header from "./components/Header.vue";
import ModalLoading from "./components/commons/ModalLoading.vue";
import { mapGetters } from "vuex";

export default defineComponent({
    name: "App",
    components: {
        Header,
        ModalLoading,
    },
    computed: {
        ...mapGetters(["loading"]),
    },
});
</script>

<style lang="scss">
@import "./style/tokens.scss";

/* Main Style */
body {
    margin: 0 auto;
    font-family: "Poppins", sans-serif;
    color: $base-text;
    background-color: $background;
    min-width: 1000px;

    @media only screen and (max-width: 550px) {
        min-width: 0px;
    }
}

button {
    height: 40px;
    border-radius: 8px;
    font-family: "Poppins", sans-serif;
    font-weight: 700;
    cursor: pointer;
}

::selection {
    color: $base-text;
    background-color: $primary;
}
</style>
