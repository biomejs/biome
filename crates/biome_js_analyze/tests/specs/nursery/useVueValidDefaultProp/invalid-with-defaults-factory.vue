<script setup lang="ts">
/* should generate diagnostics */
withDefaults(defineProps<{ foo?: string; bar?: number }>(), {
  foo: () => 123,
  bar() {
    return "bar";
  },
});
</script>
