<!-- should not generate diagnostics -->
<script>
export default {
  emits: [
    // biome-ignore lint/nursery/noVueUnusedEmitDeclarations: emitted by a plugin
    "foo",
  ],
};
</script>
