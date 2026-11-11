<script setup>
/* should generate diagnostics */
defineProps({
  a: {
    type: 'String',
    default: 'abc',
  },
  b: ['Number', Boolean],
});
</script>
