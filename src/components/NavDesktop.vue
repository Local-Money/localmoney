<template>
  <nav>
    <ul>
      <li class="item">
        <router-link to="/explore">
          <svg class="icon-24" viewBox="0 0 24 24" fill="none">
            <path
              d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M16.24 7.76L14.12 14.12L7.76 16.24L9.88 9.88L16.24 7.76Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          <p>Explore</p>
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
    </ul>
  </nav>
</template>

<script>
import { defineComponent } from "vue";

export default defineComponent({
  name: "NavDesktop",
});
</script>

<style lang="scss" scoped>
@import "../style/tokens.scss";

nav {
  display: flex;
  justify-content: space-between;
  margin-left: auto;

  ul {
    display: flex;
    margin-right: 32px;

    li {
      padding: 0 20px;
    }

    a {
      display: inline-block;
      height: 40px;
      font-size: 14px;
      color: $base-text;

      &:hover {
        color: $gray900;
        .icon-24 {
          stroke: $primary;
        }
      }

      .icon-24 {
        vertical-align: middle;
      }

      p {
        display: inline-block;
        vertical-align: middle;
        line-height: 40px;
        margin-left: 16px;
      }
    }
  }
}
</style>
