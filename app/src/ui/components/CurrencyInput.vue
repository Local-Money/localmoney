<script>
import useCurrencyInput from 'vue-currency-input'

export default {
  name: 'CurrencyInput',
  props: ['modelValue', 'options', 'placeholder'],
  setup(props) {
    const { formattedValue, inputRef, setValue } = useCurrencyInput(props.options)

    return { inputRef, formattedValue, setValue }
  },
  methods: {
    focus() {
      const input = this.$refs.inputRef
      input.focus()
    },
    update(val) {
      this.setValue(val)
    },
  },
}
</script>

<template>
  <input ref="inputRef" :value="formattedValue" :placeholder="placeholder">
</template>
