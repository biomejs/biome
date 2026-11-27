<script setup lang="ts">
/* should generate diagnostics */
interface Props {
  greeting_text: number;
  'greeting-text'?: string;
  OnClick(): void;
}
defineProps<Props>();
</script>
