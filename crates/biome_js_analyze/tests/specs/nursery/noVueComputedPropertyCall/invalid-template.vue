<!-- should generate diagnostics -->
<template>
  <p>{{ total() }}</p>
  <button @click="total()">Log</button>
  <p v-if="label()">{{ label }}</p>
</template>

<script>
export default {
  props: { title: String },
  computed: {
    total() {
      return 42;
    },
    label() {
      return this.title;
    },
  },
};
</script>
