<script>
/* should not generate diagnostics */
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
