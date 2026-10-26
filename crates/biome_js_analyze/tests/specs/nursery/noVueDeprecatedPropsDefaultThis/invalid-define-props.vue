<script setup>
/* should generate diagnostics */
defineProps({
  a: String,
  b: {
    type: Object,
    default() {
      return { a: this.a };
    },
  },
});
</script>
