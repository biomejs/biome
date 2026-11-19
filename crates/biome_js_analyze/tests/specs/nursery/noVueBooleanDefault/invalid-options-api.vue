<script lang="ts">
/* should generate diagnostics */
export default {
  props: {
    foo: {
      type: Boolean,
      default: true,
    },
    bar: {
      type: Boolean,
      default: false,
    },
    baz: {
      type: Boolean as PropType<boolean>,
      default() {
        return true;
      },
    },
  },
};
</script>
