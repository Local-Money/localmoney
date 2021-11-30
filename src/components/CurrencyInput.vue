<template>
  <input ref="inputRef" :value="formattedValue" :placeholder="placeholder" />
</template>

<script>
import useCurrencyInput from "vue-currency-input";

export default {
  name: "CurrencyInput",
  props: ["modelValue", "options", "placeholder"],
  methods: {
    focus: function() {
      let input = this.$refs.inputRef;
      input.focus();
    },
    update(val) {
      this.setValue(val);
    },
  },
  setup(props) {
    const { formattedValue, inputRef, setValue } = useCurrencyInput(
      props.options
    );

    return { inputRef, formattedValue, setValue };
  },
};
</script>
