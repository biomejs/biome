<script>
/* should generate diagnostics */
export default {
  setup: function (props, { emit: fire }) {
    fire("click");
    fire("keydown");
  },
};
</script>
