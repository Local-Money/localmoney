<script>
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'NavMobile',
})
</script>

<template>
  <nav>
    <ul>
      <li class="item">
        <router-link to="/">
          <svg class="icon-24" viewBox="0 0 24 24" fill="none">
            <path
              d="M3 9L12 2L21 9V20C21 20.5304 20.7893 21.0391 20.4142 21.4142C20.0391 21.7893 19.5304 22 19 22H5C4.46957 22 3.96086 21.7893 3.58579 21.4142C3.21071 21.0391 3 20.5304 3 20V9Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M9 22V12H15V22"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </router-link>
      </li>

      <li class="item">
        <router-link to="/offers">
          <svg class="icon-24" viewBox="0 0 24 24" fill="none">
            <path
              d="M12 2L2 7L12 12L22 7L12 2Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M2 17L12 22L22 17"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M2 12L12 17L22 12"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
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
        </router-link>
      </li>
    </ul>
  </nav>
</template>

<style lang="scss" scoped>
@import "../style/tokens.scss";

nav {
    position: fixed;
    width: 100%;
    bottom: 0;
    padding: 16px 16px;
    height: 64px;
    z-index: 9999;
    display: flex;
    justify-content: space-between;
    margin-left: auto;
    background-color: $surface;
    border-top: 1px solid $border;

    ul {
        width: 100%;
        height: 24px;
        display: flex;
        justify-content: space-around;

        li {
        }

        a {
            &:hover {
                color: $gray900;
                .icon-24 {
                    stroke: $primary;
                }
            }

            .icon-24 {
                vertical-align: middle;
            }
        }
    }
}
</style>
