<!-- should generate diagnostics -->
<script>
export default {
  data() {
    return { price: 1 };
  },
  filters: {
    currency(value) {
      return `$${value}`;
    },
  },
};
</script>

<template>
  <p>{{ price | currency }}</p>
</template>
