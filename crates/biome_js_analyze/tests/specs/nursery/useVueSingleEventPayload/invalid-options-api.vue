<!-- should generate diagnostics -->
<script>
export default {
  emits: {
    change: (value1, value2) => true,
    update(value1, value2) {
      return true;
    },
    remove: function (value1, value2, value3) {
      return true;
    },
  },
  methods: {
    onChange() {
      this.$emit("change", value1, value2);
    },
    onUpdate() {
      this.$emit?.("update", value1, value2);
    },
  },
};
</script>
