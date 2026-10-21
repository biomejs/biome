<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    const { a } = this.$refs;
  },
};
</script>

<template>
  <input ref="a" />
</template>
