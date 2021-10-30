<template>
  <main>
    <h3>Breadcrumb > Buying UST from sambarbosa</h3>
    <section class="stepper card">
      <div class="step-item">
        <div class="icon done">
          <svg
            class="icon-24"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
          >
            <path
              d="M20 6L9 17L4 12"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </div>
        <p>seller puts UST in escrow</p>
      </div>
      <div class="step-item">
        <div class="icon">
          <div class="counter">
            <p>2</p>
          </div>
        </div>
        <p>buyer pays seller directly</p>
      </div>
      <div class="step-item">
        <div class="icon">
          <div class="counter">
            <p>3</p>
          </div>
        </div>
        <p>escrow released to buyer</p>
      </div>
      <div class="step-status">
        <div class="separator"></div>
        <div class="wrap">
          <p>time remaining</p>
          <p class="step-time-left">30 min</p>
        </div>
        <div class="icon">
          <svg
            class="icon-24"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
          >
            <path
              d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M12 6V12L16 14"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </div>
      </div>
    </section>
    <section class="wrap">
      <section class="chat card">Chat will be here</section>
      <section class="trade-summary card">
        <div class="trader-info">
          <p class="trader">sambarbosa</p>
          <p class="rating">420 trades</p>
        </div>
        <div class="trade-info">
          <p class="label">UST Price</p>
          <div class="current-price">
            <p class="ticker">Will refresh in 47s</p>
            <p class="mkt-rate">4% above market</p>
            <p class="price">COP$ 3,659.00</p>
          </div>
          <p class="label">Transaction summary</p>
          <div class="transaction">
            <div class="list-item">
              <p class="list-item-label">You will get</p>
              <p class="value">100.00 UST</p>
            </div>
            <div class="list-item">
              <p class="list-item-label">You will get</p>
              <p class="value fiat">369,559.00 COP</p>
            </div>
          </div>
        </div>
      </section>
    </section>
  </main>
</template>

<script></script>

<style lang="scss" scoped>
@import "../style/pages.scss";

.stepper {
  display: flex;
  justify-content: space-between;
  padding: 24px 40px;
  margin-bottom: 24px;
}

.step-item,
.step-status {
  width: 20%;
  display: flex;
  align-items: center;
}

.step-item {
  .icon {
    margin-right: 24px;
  }
  .done {
    svg {
      stroke: $primary;
    }
  }
  .counter {
    width: 32px;
    height: 32px;
    border-radius: 100px;
    background-color: $border;
    text-align: center;
    padding-top: 6px;
    font-size: 14px;
    font-weight: $semi-bold;
  }
  p {
    font-size: 14px;
  }
}
.step-status {
  justify-content: flex-end;
  border-left: 1px solid $border;
  .wrap {
    text-align: right;
    margin-right: 24px;
    display: flex;
    flex-direction: column;

    p {
      font-size: 14px;
      color: $gray900;
    }
    .step-time-left {
      font-size: 18px;
      font-weight: $semi-bold;
      color: $primary;
    }
  }
}

.wrap {
  display: flex;
}

.chat {
  width: 30%;
  margin-right: 24px;
}

.trade-summary {
  width: 70%;
  display: flex;
  justify-content: space-evenly;

  .label {
    margin-bottom: 8px;
    font-size: 14px;
    color: $gray900;
  }

  .trader-info {
    width: 40%;
    .trader {
      font-size: 18px;
      font-weight: $semi-bold;
    }
    .rating {
      font-size: 14px;
      color: $gray900;
    }
  }

  .trade-info {
    width: 100%;
    .current-price,
    .transaction {
      background-color: $gray150;
      padding: 16px;
      border-radius: 8px;
    }
    .current-price {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 24px;
      .ticker {
        font-size: 12px;
        color: $gray900;
      }
      .mkt-rate {
        font-size: 14px;
        color: $gray900;
      }
      .price {
        font-size: 16px;
        font-weight: $semi-bold;
      }
    }
    .transaction {
      .list-item {
        display: flex;
        justify-content: space-between;
        align-items: center;

        &:first-child {
          margin-bottom: 8px;
        }

        p {
          font-size: 16px;
        }
        .value {
          font-weight: $semi-bold;
        }
        .fiat {
          color: $primary;
        }
      }
    }
  }
}
</style>
