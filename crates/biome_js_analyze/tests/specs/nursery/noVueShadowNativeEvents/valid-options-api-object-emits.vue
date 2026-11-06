<!-- should not generate diagnostics -->
<script>
export default {
  emits: {
    welcome: null,
    save() {
      return true;
    },
    ...others,
    [dynamic]: null,
  },
};
</script>
