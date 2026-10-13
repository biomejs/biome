<!-- should generate diagnostics -->
<script>
export default {
  setup(props, context) {
    context.emit("click");
    context?.emit?.("keydown");
    (context?.emit)?.("keyup");
  },
};
</script>
