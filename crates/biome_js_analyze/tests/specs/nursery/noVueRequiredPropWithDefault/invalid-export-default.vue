<!-- should generate diagnostics -->
<script>
export default {
  props: {
    name: {
      required: true,
      default: "Hello",
    },
    "other-name": {
      required: /* comment */ true,
      default: "Hello",
    },
  },
};
</script>
