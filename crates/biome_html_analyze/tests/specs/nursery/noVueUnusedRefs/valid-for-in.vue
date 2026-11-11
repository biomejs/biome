<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    for (const key in this.$refs) {
      console.log(key);
    }
  },
};
</script>

<template>
  <input ref="a" />
</template>
