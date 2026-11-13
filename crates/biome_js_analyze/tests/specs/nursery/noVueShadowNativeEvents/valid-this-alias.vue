<!-- should not generate diagnostics -->
<!-- The rule doesn't follow variables that hold `this`. -->
<script>
export default {
  methods: {
    onAlias() {
      const vm = this;
      vm.$emit("blur");
      (vm?.$emit)?.("submit");
    },
  },
};
</script>
