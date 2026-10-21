<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    const refs = this.$refs;
    refs.a.focus();
  },
};
</script>

<template>
  <input ref="a" />
</template>
