<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('bar')">Bar</button>
</template>

<script>
export default {
  emits: ["foo", "bar"],
  methods: {
    setFoo() {
      this.$emit("foo", newFoo);
    },
  },
};
</script>
