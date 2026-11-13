<!-- should not generate diagnostics -->
<script setup lang="ts">
const props = withDefaults(defineProps<{ ["title"]: string }>(), {
  title: "Title",
});
</script>
