<!-- should generate diagnostics -->
<template>
  <div v-on="$listeners" />
  <div v-for="listener in $listeners" />
  <div :foo="$listeners" />
  <p>{{ $listeners.click }}</p>
  <button @click="$listeners = replacement" />
</template>

<script>
export default {
  computed: {
    foo() {
      fn(this.$listeners);
      return this.$listeners;
    },
    bar() {
      const vm = this;
      function inner() {
        return vm.$listeners;
      }
      const a = vm?.$listeners;
      const b = this?.$listeners;
      (this).$listeners = {};
      return inner();
    },
  },
};
</script>
