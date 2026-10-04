<script>
/* should not generate diagnostics */
export default {
  setup(props, ...context) {
    context.emit("click");
  },
};
</script>
