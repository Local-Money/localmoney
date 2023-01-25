<script setup>
import { formatAmount } from '~/shared'
const props = defineProps(['modelValue', 'options', 'placeholder', 'prefix', 'isCrypto', 'decimals'])
const emit = defineEmits(['update:modelValue'])
// create a data object with the data object with value as property.
const value = ref(props.modelValue)
const placeholder = ref(props.placeholder)
const isCrypto = ref(props.isCrypto)
const decimals = ref(props.decimals)
const formattedValue = ref(formatAmount(props.modelValue, isCrypto.value, decimals.value))
const watching = ref(false)
const inputRef = ref('')

watch(
  () => props.modelValue,
  (newValue) => {
    if (!watching.value) {
      format(Number(newValue))
    }
  }
)

function format(newValue) {
  if (isCrypto.value) {
    formattedValue.value = `${props.prefix} ${parseFloat(
      formatAmount(newValue * 1000000, isCrypto.value, decimals.value),
      decimals.value
    )}`
  } else {
    formattedValue.value = `${props.prefix} ${parseFloat(
      formatAmount(newValue, isCrypto.value, decimals.value),
      decimals.value
    )}`
  }
}

function onFocus() {
  watching.value = true
  formattedValue.value = formattedValue.value.replace(/[^0-9.]/g, '')
  value.value = formattedValue.value
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
  format(Number(value.value))
}

onMounted(() => {
  inputRef.value.focus()
  format(Number(value.value))
})
</script>

<template>
  <input
    ref="inputRef"
    :value="formattedValue"
    :placeholder="placeholder"
    type="text"
    @input="onChange"
    @focus="onFocus"
    @blur="onBlur"
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
