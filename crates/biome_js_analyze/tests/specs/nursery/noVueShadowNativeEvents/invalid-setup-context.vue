<script>
/* should generate diagnostics */
export default {
  setup(props, context) {
    context.emit("click");
    context?.emit?.("keydown");
    (context?.emit)?.("keyup");
  },
};
</script>
