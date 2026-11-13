<!-- should generate diagnostics -->
<script>
export default {
  emits: ["click", "Keydown", `input`],
  methods: {
    onClick() {
      this.$emit("change");
      this?.$emit?.("focus");
    },
  },
};
</script>
