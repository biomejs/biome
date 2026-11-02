<script setup lang="ts">
/* should not generate diagnostics */
withDefaults(defineProps<{ foo?: string }>(), {
  foo: "foo",
} as const);
</script>
