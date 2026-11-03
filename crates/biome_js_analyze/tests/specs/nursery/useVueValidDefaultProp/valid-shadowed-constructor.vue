<script>
/* should not generate diagnostics */
function Number() {
  return "not a number";
}

export default {
  props: {
    foo: { type: String, default: Number() },
  },
};
</script>
