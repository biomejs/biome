<!-- should not generate diagnostics -->
<script>
export default {
  emits: ["foo"],
  methods: {
    click() {
      const vm = this;
      vm?.$emit?.("foo");
    },
  },
};
</script>
