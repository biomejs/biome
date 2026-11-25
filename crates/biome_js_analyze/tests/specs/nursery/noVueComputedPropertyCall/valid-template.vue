<!-- should not generate diagnostics -->
<template>
  <p>{{ total }}</p>
  <p>{{ formatter(total) }}</p>
  <p>{{ total.toFixed(2) }}</p>
  <button @click="formatter">Log</button>
</template>

<script>
export default {
  computed: {
    total() {
      return 42;
    },
    formatter() {
      return (value) => `${value}`;
    },
  },
};
</script>
