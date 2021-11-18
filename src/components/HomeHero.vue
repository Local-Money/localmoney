<template>
  <section>
    <div class="title-column">
      <h1 class="title">Welcome to Local Terra.</h1>
      <h2 class="sub-title">
        We are a decentralized P2P marketplace for
        <a href="https://www.terra.money/" target="_blank">@terra_money</a>
        assets.
      </h2>
    </div>

    <div class="card-column-1">
      <div class="card currency">
        <img src="@/assets/ic_ust.svg" alt="Icon UST" />
        <p class="token">UST</p>
        <p class="price">${{this.ustUsdPrice}} USD</p>
      </div>

      <div class="card currency">
        <img src="@/assets/ic_luna.svg" alt="Icon UST" />
        <p class="token">Luna</p>
        <p class="price">{{this.lunaUstPrice}} UST</p>
      </div>
    </div>

    <div class="card-column-2">
      <div class="card big">
        <img src="@/assets/ic_anchor.svg" alt="Icon UST" />
        <p class="content">Better savings with Anchor</p>
        <div class="yield">
          <p class="caption">yearly</p>
          <p class="percentage">19.72%</p>
        </div>
        <a href="#" @click="alert('soon :)')">connect ></a>
      </div>
    </div>
  </section>
</template>

<script>
import { defineComponent } from "vue";
import { mapActions, mapGetters } from "vuex";
import { formatAmount } from "@/shared";

export default defineComponent({
  name: "HomeHero",
  methods: {
    ...mapActions(["fetchLunaPrice", "fetchUstUsdPrice"]),
    formatAmount,
    alert: function (msg) {
      window.alert(msg)
    }
  },
  computed: {
    ...mapGetters(["lunaUstPrice", "ustUsdPrice"])
  },
  mounted: function () {
    this.fetchLunaPrice()
    this.fetchUstUsdPrice()
  }
})
</script>

<style lang="scss" scoped>
@import "../style/tokens.scss";

section {
  display: grid;
  grid-template-columns: 50% 1fr 1fr;
  gap: 16px;
  padding: 80px 0;

  @media only screen and (max-width: 1100px) {
    grid-template-columns: 35% 1fr 1fr;
  }

  @media only screen and (max-width: 550px) {
    grid-template-columns: 1fr;
    padding: 32px 0 24px;
  }

  a {
    color: $primary;
    text-decoration: none;
  }
}

.title-column {
  grid-column: 1/2;
  grid-row: 1;
  padding-right: 24px;
}
.card-column-1 {
  grid-column: 3/4;
  grid-row: 1;
}
.card-column-2 {
  grid-column: 4/5;
  grid-row: 1;
}

@media only screen and (max-width: 550px) {
  .title-column {
    grid-column: 1;
    grid-row: 1;
    text-align: center;
  }
  .card-column-1 {
    grid-column: 1;
    grid-row: 2;
  }
  .card-column-2 {
    grid-column: 1;
    grid-row: 3;
  }
}

/* ----------- CARDS GRID */

h1 {
  font-size: 32px;
  font-weight: 600;
  display: block;
}
h2 {
  font-size: 24px;
  font-weight: 400;
  display: block;
}

@media only screen and (max-width: 550px) {
  h1 {
    font-size: 24px;
    margin-bottom: 8px;
  }
  h2 {
    font-size: 18px;
    margin-bottom: 16px;
  }
}

.card {
  padding: 16px;
  background-color: $surface;
  border-radius: 16px;
  box-shadow: 0px 2px 25px rgba(0, 0, 0, 0.05);
  min-width: 256px;
}

.currency {
  display: grid;
  grid-template-columns: 32px 1fr 1fr;
  gap: 8px;

  img {
    width: 32px;
    margin: auto 0;
  }
  p {
    font-weight: 600;
    font-size: 16px;
    margin: auto 0;
  }
  .price {
    justify-self: end;
  }
}

.card:first-child {
  margin-bottom: 16px;
}

.big {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;

  img {
    width: 48px;
  }

  .content {
    grid-column: 2 / 4;
    grid-row: 1 / 2;
    text-align: right;
    align-self: end;
    font-size: 18px;
    font-weight: 600;
    line-height: 120%;
  }

  .yield {
    grid-column: 1 / 2;
    grid-row: 2 / 3;
    text-align: left;

    .caption {
      font-size: 12px;
      color: $gray900;
    }
    .percentage {
      font-size: 20px;
      font-weight: 800;
    }
  }

  a {
    grid-column: 2 / 4;
    grid-row: 2 / 3;
    align-self: end;
    text-align: right;
    font-size: 18px;
    font-weight: 800;
    color: $anchor;
  }
}
</style>
