<!-- should not generate diagnostics -->
<script>
export default {
  methods: {
    getRefs() {
      return this.$refs;
    },
  },
};
</script>

<template>
  <input ref="a" />
</template>
