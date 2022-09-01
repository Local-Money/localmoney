<script setup lang="ts">
import type { SelectInfo } from '~/utils/select-utils'
import { getSelectInfo } from '~/utils/select-utils'
const props = defineProps<{
  modelValue: string
  options: Map<string, SelectInfo>
}>()
const emit = defineEmits<{
  (e: 'update:modelValue', modelValue: string): void
}>()
</script>

<template>
  <v-select
    class=""
    :modelValue="modelValue"
    :options="[...options.keys()]"
    :searchable="false"
    :clearable="false"
    @update:modelValue="emit('update:modelValue', $event)"
  >
    <template #selected-option>
      <div class="wrap" v-bind="(info = getSelectInfo(options, modelValue))">
        <img v-if="info.icon" :src="info.icon" />
        <p>{{ info.display }}</p>
      </div>
    </template>
    <template #option="{ label }">
      <div class="wrap" v-bind="(info = getSelectInfo(options, label))">
        <img v-if="info.icon" :src="info.icon" />
        <p>{{ info.display }}</p>
      </div>
    </template>
  </v-select>
</template>

<style lang="scss">
@import '../../style/tokens';

.v-select {
  --vs-border-color: #33363c;
  --vs-border-radius: 8px;

  height: 40px;

  @media only screen and (max-width: $mobile) {
    height: 48px;
  }

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
    padding: 4px 8px;
  }

  .vs__dropdown-option--highlight {
    background: $gray300;
  }

  .vs__actions {
    padding: 4px 4px;
    svg {
      fill: $gray600;
    }
  }

  .wrap {
    display: flex;
    align-content: space-between;
    align-items: center;
    height: 24px;
    img {
      width: 24px;
    }
    p {
      margin-left: 10px;
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
    position: relative;
  }
}
.vs--single.vs--open .vs__selected {
  position: relative;
}
</style>
