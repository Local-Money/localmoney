<template>
  <section>
    <div class="title-column">
      <h1 class="title">Welcome to Local Terra.</h1>
      <h2 class="sub-title">
        We are a decentralized P2P marketplace for
        <a href="https://www.terra.money/" target="_blank">@terra_money</a> assets.
      </h2>
    </div>

    <div class="card-column-1">
      <div class="card currency">
        <img src="@/assets/ic_ust.svg" alt="Icon UST" />
        <p class="token">UST</p>
        <p class="price">$1.01 USD</p>
      </div>

      <div class="card currency">
        <img src="@/assets/ic_luna.svg" alt="Icon UST" />
        <p class="token">Luna</p>
        <p class="price">$11.78 UST</p>
      </div>
    </div>

    <div class="card-column-2">
      <div class="card big">
        <img src="@/assets/ic_anchor.svg" alt="Icon UST" />
        <p class="content">Better savings with Anchor</p>
        <div class="yield">
          <p class="caption">yearly</p>
          <p class="percentage">19.86%</p>
        </div>
        <a href="#">connect ></a>
      </div>
    </div>
  </section>
</template>

<style lang="scss" scoped>
@import '../style/header.scss';
</style>
