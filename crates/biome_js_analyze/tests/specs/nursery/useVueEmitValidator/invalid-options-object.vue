<script lang="ts">
/* should generate diagnostics */
export default {
  emits: {
    skipped: null,
    alsoSkipped: undefined,
    parenthesized: (null),
    withComment: /* no validation */ null,
    objectValue: {
      type: String,
    },
    asserted: {} as ((payload: string) => boolean),
    "update:modelValue": null,
  },
};
</script>
