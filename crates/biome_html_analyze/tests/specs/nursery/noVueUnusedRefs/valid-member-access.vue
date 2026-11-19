<!-- should not generate diagnostics -->
<script>
export default {
  mounted() {
    this.$refs.a;
    this.$refs?.b;
    this?.$refs.c;
    this?.$refs?.d;
    (this?.$refs)?.e;
    this.$refs["f"].focus();
  },
};
</script>

<template>
  <input ref="a" />
  <input ref="b" />
  <input ref="c" />
  <input ref="d" />
  <input ref="e" />
  <input ref="f" />
</template>
