<!-- should not generate diagnostics -->
<script>
export default {
  emits: { foo: null },
  setup(_, { emit }) {
    const setFoo = () => {
      emit("foo", newFoo);
    };
    return { setFoo };
  },
};
</script>
