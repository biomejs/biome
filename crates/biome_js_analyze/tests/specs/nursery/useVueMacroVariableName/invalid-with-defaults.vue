<script setup lang="ts">
/* should generate diagnostics */
const propsWrong = withDefaults(defineProps<{ foo?: number; bar?: string }>(), {
  foo: 42,
  bar: "abc",
});
</script>
