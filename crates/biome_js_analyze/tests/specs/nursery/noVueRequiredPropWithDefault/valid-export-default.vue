<!-- should not generate diagnostics -->
<script>
export default {
  props: {
    name: {
      required: false,
      default: "Hello",
    },
    age: {
      type: Number,
      required: true,
    },
  },
};
</script>
