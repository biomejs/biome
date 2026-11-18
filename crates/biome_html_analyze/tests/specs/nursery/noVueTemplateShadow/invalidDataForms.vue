<!-- should generate diagnostics -->
<template>
  <div v-for="e in f" />
  <div v-for="f in c" />
  <div v-for="g in c" />
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
    e,
    ["f"]: [1, 2],
  },
};
</script>
