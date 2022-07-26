<script setup lang="ts">
import { fiatsAvailable, getFiatInfo } from "~/utils/fiat";
</script>

<template>
  <v-select
    id="currency"
    class=""
    name="currency"
    v-model="fiatCurrency"
    :options="[...fiatsAvailable.keys()]"
    :searchable="false"
    :clearable="false"
  >
    <template #selected-option>
      <div class="wrap">
        <img :src="getFiatInfo(fiatCurrency).flag" />
        <p>{{ getFiatInfo(fiatCurrency).display }}</p>
      </div>
    </template>
    <template #option="{ label }">
      <div class="wrap">
        <img :src="getFiatInfo(label).flag" />
        <p>{{ getFiatInfo(label).display }}</p>
      </div>
    </template>
  </v-select>
</template>

<style lang="scss">
@import "../../style/tokens";

.v-select {
  --vs-border-color: #33363c;
  --vs-border-radius: 8px;

  height: 48px;

  .vs__dropdown-menu {
    background: $surface;

    li {
      margin: 4px 0;
      padding: 8px 12px;
    }
  }

  .vs__dropdown-toggle {
    background: $surface;
    height: inherit;
    margin: 0;
    padding: 8px;
  }

  .vs__dropdown-option--highlight {
    background: $gray300;
  }

  .vs__actions {
    svg {
      fill: $gray600;
      stroke: 1px;
    }
  }

  .wrap {
    display: flex;
    align-content: space-between;
    align-items: center;
    img {
      width: 24px;
    }
    p {
      margin-left: 8px;
      font-size: 14px;
      font-weight: 600;
      color: $base-text;
      line-height: 0;
    }
  }
}

//RESETS
.v-select {
  .vs__selected,
  .vs__search {
    height: inherit;
    margin: 0;
    padding: 0;
  }
  .vs--single.vs--open .vs__selected {
    display: relative;
  }
}
.vs--single.vs--open .vs__selected {
  position: relative;
}
</style>
