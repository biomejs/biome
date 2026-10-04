<script>
/* should not generate diagnostics */
export default {
  setup(props, { emit, expose }) {
    emit("welcome");
    expose("click");
    emit = "click";
  },
};
</script>
