/* should not generate diagnostics */
export default {
  emits: ["click"],
  methods: {
    onClick() {
      this.$emit("click");
    },
  },
};

$emit("click");
