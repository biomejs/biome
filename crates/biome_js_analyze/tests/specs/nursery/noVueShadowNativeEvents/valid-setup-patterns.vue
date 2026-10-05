<!-- should not generate diagnostics -->
<script>
export default {
  setup(props, ...context) {
    context.emit("click");
  },
};
</script>
