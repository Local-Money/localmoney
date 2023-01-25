<script setup>
import { formatAmount } from '~/shared'
const props = defineProps(['modelValue', 'options', 'placeholder', 'prefix'])
const emit = defineEmits(['update:modelValue'])
// create a data object with the data object with value as property.
const value = ref(props.modelValue)
const placeholder = ref(props.placeholder)
const formattedValue = ref(formatAmount(props.modelValue))
const watching = ref(false)

watch(
  () => props.modelValue,
  (newValue) => {
    if (!watching.value) {
      formattedValue.value = `${props.prefix} ${formatAmount(newValue * 1000000)}`
    }
  }
)

function focus() {
  watching.value = true
  formattedValue.value = formattedValue.value.replace(/[^0-9.]/g, '')
}

function onChange(e) {
  const newValue = e.target.value.replace(/[^0-9.]/g, '')
  value.value = newValue
  if (watching.value) {
    formattedValue.value = value.value
  }
  emit('update:modelValue', newValue)
}

function onBlur() {
  watching.value = false
  formattedValue.value = `${props.prefix} ${formatAmount(value.value * 1000000)}`
}
</script>

<template>
  <input
    ref="inputRef"
    :value="formattedValue"
    :placeholder="placeholder"
    type="text"
    @input="onChange"
    @focus="focus"
    @blur="onBlur()"
  />
</template>

<style lang="scss">
/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type='number'] {
  -moz-appearance: textfield;
}
</style>
