<!-- should not generate diagnostics -->
<script setup>
defineProps({
  add: {
    type: Function,
    default(a, b) {
      const sum = a + b;
      return sum;
    },
  },
  label: {
    type: String,
    default: () => window.location.hostname,
  },
});
</script>
