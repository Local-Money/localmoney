<script setup lang="ts">
//TODO clear unused method

const widgetActive = ref(false);
function toggleWidget() {
  widgetActive.value = !widgetActive.value;
}

// const widgetActive = ref(false);
// function hoverIn() {
//   widgetActive.value = true;
// }
// function hoverOut() {
//   widgetActive.value = false;
// }
</script>

<template>
  <div class="wrap-widget">
    <div class="wrap-btn" @click="toggleWidget">
      <div class="badge">
        <p>2</p>
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
    <div class="widget" v-if="widgetActive">
      <div class="header">
        <p class="title">Notifications</p>
        <p class="mark-read">Mark all as read</p>
      </div>
      <div class="content">
        <ul>
          <li class="item">
            <svg class="icon" viewBox="0 0 24 24" fill="none">
              <path
                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 8L12.01 12"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 16L12 16"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
            <div class="wrap">
              <p class="status">You have a new trade request</p>
              <p class="addr">from kujira6...6yp193</p>
            </div>
            <p class="timestamp">2h ago</p>
          </li>
          <li class="item">
            <svg class="icon" viewBox="0 0 24 24" fill="none">
              <path
                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 8L12.01 12"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 16L12 16"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
            <div class="wrap">
              <p class="status">You have a new trade request</p>
              <p class="addr">from kujira6...6yp193</p>
            </div>
            <p class="timestamp">2h ago</p>
          </li>
          <li class="item">
            <svg class="icon" viewBox="0 0 24 24" fill="none">
              <path
                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 8L12.01 12"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 16L12 16"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
            <div class="wrap">
              <p class="status">You have a new trade request</p>
              <p class="addr">from kujira6...6yp193</p>
            </div>
            <p class="timestamp">2h ago</p>
          </li>
          <li class="item">
            <svg class="icon" viewBox="0 0 24 24" fill="none">
              <path
                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 8L12.01 12"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 16L12 16"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
            <div class="wrap">
              <p class="status">You have a new trade request</p>
              <p class="addr">from kujira6...6yp193</p>
            </div>
            <p class="timestamp">2h ago</p>
          </li>
          <li class="item">
            <svg class="icon" viewBox="0 0 24 24" fill="none">
              <path
                d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 8L12.01 12"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <path
                d="M12.01 16L12 16"
                stroke="inherit"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
            <div class="wrap">
              <p class="status">You have a new trade request</p>
              <p class="addr">from kujira6...6yp193</p>
            </div>
            <p class="timestamp">2h ago</p>
          </li>
        </ul>
      </div>
    </div>
    <div class="widget-close" @click="toggleWidget" v-if="widgetActive" />
  </div>
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.wrap-widget {
  display: flex;
  flex-direction: row-reverse;

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
      right: 10px;
      background: $primary;
      border-radius: 56px;

      p {
        font-size: 12px;
        font-weight: $semi-bold;
        color: $base-text;
      }
    }
    .btn {
      width: 40px;
      height: 40px;
      background: $surface;
      border-radius: 56px;
      display: flex;
      align-items: center;
      justify-content: center;
      margin-right: 24px;

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

  .widget {
    position: absolute;
    min-width: 400px;
    margin-top: 64px;
    margin-right: -72px;
    z-index: 100;
    background-color: $surface;
    border: 1px solid $border;
    border-radius: 8px;
    box-shadow: 0px 2px 25px rgba(0, 0, 0, 0.05);

    .header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 16px 24px;
      border-bottom: 1px solid $border;

      .title {
        font-size: 18px;
        font-weight: $semi-bold;
      }
      .mark-read {
        font-size: 12px;
        text-decoration: underline;
        color: $gray700;
        cursor: pointer;
      }
    }

    .content {
      max-height: 300px;
      background: $gray200;
      overflow-y: scroll;
      &::-webkit-scrollbar {
        width: 8px;
        background: $surface;
      }
      &::-webkit-scrollbar-thumb {
        background: $gray300;
        border-radius: 56px;
      }
      .item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 16px;
        font-size: 14px;
        padding: 16px 24px;
        cursor: pointer;
        z-index: 100;

        &:hover {
          background: $gray150;
        }
        &:last-child {
          margin-bottom: 8px;
        }

        .icon {
          width: 20px;
          height: 20px;
          stroke: $primary;
        }

        .wrap {
          flex-grow: 2;
        }

        .addr {
          font-size: 14px;
          color: $gray700;
        }
        .timestamp {
          font-size: 12px;
          color: $gray900;
        }
      }
    }
  }
  .widget-close {
    position: fixed;
    width: 100%;
    height: 100vh;
    left: 0;
    top: 0;
    z-index: 99;
  }
}
</style>
