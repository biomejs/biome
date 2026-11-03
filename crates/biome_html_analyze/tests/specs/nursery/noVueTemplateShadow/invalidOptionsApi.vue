<!-- should generate diagnostics -->
<template>
  <div v-for="count in 5"></div>
  <div v-for="label in 5"></div>
  <div v-for="total in 5"></div>
  <div v-for="reset in 5"></div>
  <div v-for="fetched in 5"></div>
  <div v-for="title in 5"></div>
  <div v-for="exposed in 5"></div>
  <div v-for="theme in 5"></div>
</template>

<script>
export default {
  props: ["title"],
  inject: ["theme"],
  data() {
    return {
      count: 0,
      label: "",
    };
  },
  asyncData() {
    return {
      fetched: true,
    };
  },
  computed: {
    total() {
      return this.count;
    },
  },
  methods: {
    reset() {},
  },
  setup() {
    return {
      exposed: 42,
    };
  },
};
</script>
