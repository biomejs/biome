<!-- should not generate diagnostics -->
<template>
  <div v-for="i in list"></div>
  <div v-for="name in list"></div>
  <div v-for="mounted in list"></div>
  <div v-for="local in list"></div>
</template>

<script>
const local = 1;

export default {
  data() {
    return {
      list: [1, 2, 3],
      user: { name: "" },
    };
  },
  watch: {
    list() {},
  },
  mounted() {},
};
</script>
