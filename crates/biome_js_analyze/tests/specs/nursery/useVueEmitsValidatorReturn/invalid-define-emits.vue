<script setup lang="ts">
/* should generate diagnostics */
defineEmits({
  foo() {
  },
  bar: (value: string) => false as boolean,
});
</script>
