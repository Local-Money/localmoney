<script setup lang="ts">
import { formatAmount } from '~/shared'
const props = defineProps<{
  modelValue: number
  placeholder: string
  isCrypto: boolean
  decimals: number
  prefix?: string
  min?: number
  max?: number
  errorMsg?: string
}>()

const emit = defineEmits(['update:modelValue'])
// create a data object with the data object with value as property.
const value = ref(props.modelValue)
const placeholder = ref(props.placeholder)
const isCrypto = ref(props.isCrypto)
const decimals = ref(props.decimals)
const formattedValue = ref(formatAmount(props.modelValue, isCrypto.value, decimals.value))
const watching = ref(false)
const inputRef = ref('')
const error = ref(false)

watch(
  () => props.modelValue,
  (newValue) => {
    if (!watching.value) {
      format(Number(newValue))
    }
    value.value = newValue
    error.value = (props.min && newValue < props.min) || (props.max && newValue > props.max)
  }
)

function format(newValue: any) {
  if (newValue === undefined || newValue === null || isNaN(newValue)) {
    newValue = '0'
  }
  if (isCrypto.value) {
    formattedValue.value = `${props.prefix} ${parseFloat(
      formatAmount(newValue * 1000000, isCrypto.value, decimals.value)
    )}`
  } else {
    formattedValue.value = `${props.prefix} ${parseFloat(formatAmount(newValue, isCrypto.value, decimals.value))}`
  }
}

function onFocus() {
  watching.value = true
  formattedValue.value = formattedValue.value.replace(/[^0-9.]/g, '')
  value.value = formattedValue.value
}

function onChange(e: { target: { value: string } }) {
  const newValue = e.target.value.replace(/[^0-9.]/g, '')
  if (newValue === '') {
    value.value = 0
    return
  }
  value.value = parseFloat(newValue)
  if (watching.value) {
    formattedValue.value = value.value
  }
  emit('update:modelValue', value.value)
}

function onBlur() {
  watching.value = false
  format(Number(value.value))
}

onMounted(() => {
  format(Number(value.value))
})
</script>

<template>
  <input
    ref="inputRef"
    :value="formattedValue"
    :placeholder="placeholder"
    type="text"
    :class="{ error }"
    @input="onChange"
    @focus="onFocus"
    @blur="onBlur"
  />
  <!-- Uncomment to show label -->
  <!-- <label v-if="errorMsg && error" class="error-msg">{{ errorMsg }}</label> -->
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

.error {
  border: 1px solid red;
  background-color: aqua;
}
</style>
