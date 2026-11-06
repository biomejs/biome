<!-- should not generate diagnostics -->
<script>
export default {
  emits: ["count", "foo"],
  methods: {
    setCount() {
      this.$emit("count", newCount);
    },
    setFoo() {
      this.$emit("foo", newFoo);
    },
  },
};
</script>
