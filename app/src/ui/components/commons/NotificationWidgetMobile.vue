<script setup lang="ts">
//TODO clear unused method

// const widgetActive = ref(false);
// function toggleWidget() {
//   widgetActive.value = !widgetActive.value;
// }

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
    <div class="header">
      <p class="title">Notifications</p>
      <p class="mark-read">Mark all as read</p>
    </div>
    <div class="content">
      <ul>
        <li class="item">
          <svg class="icon" viewBox="0 0 24 24" fill="none">
            <path
              d="M20 6L9 17L4 12"
              stroke="inherit"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          <div class="wrap">
            <p class="status">Trade finished successfully</p>
            <p class="addr">from kujira6...6yp193</p>
          </div>
          <p class="timestamp">15s ago</p>
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
            <p class="status">Buyer is waiting for funds</p>
            <p class="addr">from kujira6...6yp193</p>
          </div>
          <p class="timestamp">35m ago</p>
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
            <p class="status">The seller is waiting for your payment</p>
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
</template>

<style lang="scss" scoped>
@import "../../style/tokens.scss";

.wrap-widget {
  position: fixed;
  width: 100%;
  height: 400px;
  right: 0;
  bottom: 0;
  padding-bottom: 400px;
  z-index: 100;
  background-color: $surface;
  border: 1px solid $border;
  border-radius: 8px;
  box-shadow: 0px 2px 25px rgba(0, 0, 0, 0.2);

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
    height: 300px;
    background: $gray200;
    overflow-y: scroll;
    overscroll-behavior: contain;
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
</style>
