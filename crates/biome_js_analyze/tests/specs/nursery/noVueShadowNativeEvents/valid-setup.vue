<script>
/* should not generate diagnostics */
export default {
  setup(props, context) {
    context.emit("welcome");
    context.fire("click");
    context.emit = "click";
    context = "click";
    emitLater(context);
  },
};
</script>
