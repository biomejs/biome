<script>
/* should generate diagnostics */
export default {
  emits: ["click", "Keydown", `input`],
  methods: {
    onClick() {
      this.$emit("change");
      this?.$emit?.("focus");
    },
    onAlias() {
      const vm = this;
      vm.$emit("blur");
      (vm?.$emit)?.("submit");
    },
  },
};
</script>
