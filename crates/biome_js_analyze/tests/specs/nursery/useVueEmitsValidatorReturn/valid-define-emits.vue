<script setup>
/* should not generate diagnostics */
defineEmits({
  foo() {
    return true;
  },
  bar: (value) => typeof value === "string",
});
</script>
