<!-- should not generate diagnostics -->
<template>
  <div v-for="i in c" />
  <div v-for="a in c" />
  <div v-for="b in c" />
  <div v-for="d in c" />
  <div v-for="nested in c" />
</template>

<script>
export default {
  ...a,
  data: () => ({
    ...b,
    c: [1, 2, 3],
    g: { nested: true },
  }),
  computed: {
    ...d,
  },
};
</script>
