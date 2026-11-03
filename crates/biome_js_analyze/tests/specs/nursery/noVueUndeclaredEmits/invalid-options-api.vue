<!-- should generate diagnostics -->
<template>
  <button @click="$emit('open')">Open</button>
</template>

<script>
export default {
  emits: ["change"],
  methods: {
    close() {
      this.$emit("close");
    },
  },
};
</script>
