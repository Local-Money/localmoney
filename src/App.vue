<template>
  <Header />
  <router-view />
</template>

<script>
import { defineComponent } from 'vue'
import Header from './components/Header.vue'

export default defineComponent({
  name: 'App',
  components: {
    Header,
  },
})
</script>

<style lang="scss">
@import url('https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600;700&display=swap');
@import './style/tokens.scss';

/* Main Style */
body {
  margin: 0 auto;
  font-family: 'Poppins', sans-serif;
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
  font-family: 'Poppins', sans-serif;
  font-weight: 700;
  cursor: pointer;
}
</style>
