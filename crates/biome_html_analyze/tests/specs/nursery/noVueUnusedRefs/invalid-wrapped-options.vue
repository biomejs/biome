<!-- should generate diagnostics -->
<script>
export default createComponent({
  mounted() {
    this.$refs.x.focus();
  },
});
</script>

<template>
  <input ref="x" />
  <input ref="y" />
</template>
