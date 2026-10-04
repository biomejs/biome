<!-- should not generate diagnostics -->
<script>
export default {
  setup(props, { emit, expose }) {
    emit("welcome");
    expose("click");
    emit = "click";
  },
};
</script>
