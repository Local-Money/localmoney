<script setup lang="ts">
interface TradeActionButton {
  label: string
  action: () => void
}

const props = defineProps<{
  message: string
  subMessage?: string
  buttons?: TradeActionButton[]
  icon?: string
}>()
</script>

<template>
  <div class="wrap">
    <div class="wrap-message">
      <BaseIcon :icon="icon ?? 'arrow'" :class="icon === 'check' ? 'done' : 'color'" />
      <p>
        {{ message }} <span v-if="subMessage">{{ subMessage }}</span>
      </p>
    </div>
    <div class="wrap-buttons">
      <button v-for="button in props.buttons" :key="button.label" class="bg-gray300 primary" @click="button.action()">
        {{ button.label }}
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import '../../style/pages';

.wrap {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 40px;

  .wrap-message,
  .wrap-buttons {
    display: flex;
    align-items: center;
    gap: 24px;
  }

  .wrap-message {
    svg {
      flex-basis: 24px;
      flex-grow: 0;
      flex-shrink: 0;
      stroke: $primary;
    }
    p {
      font-size: 16px;
      font-weight: 700;

      span {
        color: $primary;
      }
    }
  }
  .wrap-buttons {
    flex-shrink: 0;
  }
}

button {
  margin-left: auto;
}
</style>
