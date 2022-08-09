<script setup lang="ts">
const widgetActive = ref(false);
function toggleWidget() {
  widgetActive.value = !widgetActive.value;
}
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
      <li class="item">
        <div class="wrap-btn" @click="toggleWidget">
          <div class="badge">
            <p>4</p>
          </div>
          <div class="btn">
            <svg viewBox="0 0 24 24" fill="none">
              <path
                d="M18 8C18 6.4087 17.3679 4.88258 16.2426 3.75736C15.1174 2.63214 13.5913 2 12 2C10.4087 2 8.88258 2.63214 7.75736 3.75736C6.63214 4.88258 6 6.4087 6 8C6 15 3 17 3 17H21C21 17 18 15 18 8Z"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M13.73 21C13.5542 21.3031 13.3018 21.5547 12.9982 21.7295C12.6946 21.9044 12.3504 21.9965 12 21.9965C11.6496 21.9965 11.3054 21.9044 11.0018 21.7295C10.6982 21.5547 10.4458 21.3031 10.27 21"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </div>
        </div>
      </li>
    </ul>
  </nav>

  <transition name="widget-transition">
    <NotificationWidgetMobile v-if="widgetActive" />
  </transition>

  <transition name="widget-fade">
    <div class="widget-close" @click="toggleWidget" v-if="widgetActive" />
  </transition>
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
  .wrap-btn {
    position: relative;
    z-index: 100;
    cursor: pointer;
    .badge {
      width: 24px;
      height: 24px;
      display: flex;
      align-items: center;
      justify-content: center;
      align-content: flex-start;

      position: absolute;
      z-index: 101;
      top: -8px;
      right: -12px;
      background: $primary;
      border-radius: 56px;

      p {
        font-size: 12px;
        font-weight: $semi-bold;
        color: $base-text;
      }
    }
    .btn {
      width: 24px;
      height: 24px;
      background: $surface;
      border-radius: 56px;
      display: flex;
      align-items: center;
      justify-content: center;

      svg {
        width: 20px;
        height: 20px;
        stroke: $base-text;
      }
      &:hover {
        svg {
          stroke: $primary;
        }
      }
      &:active {
        transform: scale(0.9);
        z-index: 99;
      }
    }
  }
}
.widget-close {
  position: fixed;
  background: rgba($color: #000000, $alpha: 0.4);
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  z-index: 99;
}

.widget-transition-enter-active,
.widget-transition-leave-active {
  transition: all 0.3s ease;
}

.widget-transition-enter-from {
  transform: translateY(100%);
}
.widget-transition-enter-to {
}

.widget-transition-leave-from {
}
.widget-transition-leave-to {
  transform: translateY(100%);
}

// .widget-transition-enter-from,
// .widget-transition-leave-to {
//   transform: translateY(0px);
//   opacity: 0;
// }

.widget-fade-enter-active,
.widget-fade-leave-active {
  transition: opacity 0.3s;
}
.widget-fade-enter-from,
.widget-fade-leave-to {
  opacity: 0;
}
</style>
