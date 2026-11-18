<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    this.$refs[foo].value = "foo";
  },
};
</script>

<template>
  <input ref="foo" />
  <input ref="bar" />
</template>
