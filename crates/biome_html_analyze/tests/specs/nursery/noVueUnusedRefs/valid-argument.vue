<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    focusAll(this.$refs);
  },
};
</script>

<template>
  <input ref="a" />
</template>
