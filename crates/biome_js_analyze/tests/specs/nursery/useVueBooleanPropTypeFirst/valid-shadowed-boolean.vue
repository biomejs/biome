<!-- should not generate diagnostics -->
<script>
const Boolean = class {};

export default {
  props: {
    a: [String, Boolean],
  },
};
</script>
