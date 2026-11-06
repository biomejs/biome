<!-- should generate diagnostics -->
<script>
export default {
  emits: ["foo", "bar"],
  setup(_, { emit }) {
    const setFoo = () => {
      emit("foo", newFoo);
    };
    return { setFoo };
  },
};
</script>
