<!-- should generate diagnostics -->
<script>
export default {
  props: ["name"],
  data() {
    return {
      count: 0,
    };
  },
  methods: {
    click() {
      this.count++;
      this.cnt++;
    },
  },
};
</script>

<template>
  <div @click="click">{{ name }}: {{ count }}</div>
  <div :title="label">{{ cnt }}</div>
</template>
