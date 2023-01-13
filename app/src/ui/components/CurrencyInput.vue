<script>
import useCurrencyInput from 'vue-currency-input'

export default {
  name: 'CurrencyInput',
  props: {
    modelValue: Number,
    options: Object,
    placeholder: String,
    onUpdate: Function,
  },
  emits: ['onUpdate'],
  setup(props) {
    const { formattedValue, inputRef, setValue, numberValue } = useCurrencyInput(props.options)

    return { inputRef, formattedValue, setValue, numberValue }
  },
  mounted() {
    this.$refs.inputRef.addEventListener('blur', () => {
      this.$emit('onUpdate', this.numberValue)
    })
    this.$refs.inputRef.addEventListener('keyup', (e) => {
      if (e.key === 'Enter') {
        this.$emit('onUpdate', this.numberValue)
      }
    })
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
  <input ref="inputRef" :value="formattedValue" :placeholder="placeholder" />
</template>
