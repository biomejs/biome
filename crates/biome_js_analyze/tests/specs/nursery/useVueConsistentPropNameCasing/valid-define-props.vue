<script setup lang="ts">
/* should not generate diagnostics */
interface Props {
  greetingText: number;
  onClick(): void;
}
defineProps<Props>();
const greeting_text = 'local variables are not props';
</script>
