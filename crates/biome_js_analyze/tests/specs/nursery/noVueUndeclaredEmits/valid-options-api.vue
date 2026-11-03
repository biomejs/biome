<!-- should not generate diagnostics -->
<template>
  <button @click="$emit('close')">Close</button>
  <button @click="$emit(eventName)">Dynamic</button>
</template>

<script>
export default {
  emits: ["change", "close"],
  data() {
    return { eventName: "change" };
  },
  methods: {
    change() {
      this.$emit("change");
    },
  },
};
</script>
