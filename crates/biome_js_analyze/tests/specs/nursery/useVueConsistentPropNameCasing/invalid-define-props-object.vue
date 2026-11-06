<script setup>
/* should generate diagnostics */
defineProps({
  greeting_text: String,
  'greeting-text': {
    type: String,
    required: true,
  },
});
</script>
