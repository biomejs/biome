<!-- should generate diagnostics -->
<script>
export default {
  setup: function (props, { emit: fire }) {
    fire("click");
    fire("keydown");
  },
};
</script>
