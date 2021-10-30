<template>
  <input ref="inputRef" :value="formattedValue" :placeholder="placeholder" />
</template>

<script>
import useCurrencyInput from 'vue-currency-input'
import { scrollToElement } from '@/shared'

export default {
  name: 'CurrencyInput',
  props: ['modelValue', 'options', 'placeholder'],
  methods: {
    scrollToElement: scrollToElement.bind(this),
    focus: function () {
      let input = this.$refs.inputRef
      input.focus()
      this.scrollToElement(input)
    },
    update(val) {
      this.setValue(val)
    },
  },
  setup(props) {
    const { formattedValue, inputRef, setValue } = useCurrencyInput(props.options)

    return { inputRef, formattedValue, setValue }
  },
}
</script>
