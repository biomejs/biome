<!-- should generate diagnostics -->
<script>
export default {
  emits: ["foo", "count"],
  methods: {
    setFoo() {
      this.$emit("foo", newFoo);
    },
  },
};
</script>
