<script setup lang="ts">
/* should generate diagnostics */
const { greeting_text = 'hello' } = withDefaults(defineProps<{ greeting_text?: string }>(), {
  greeting_text: 'hello',
});
</script>
