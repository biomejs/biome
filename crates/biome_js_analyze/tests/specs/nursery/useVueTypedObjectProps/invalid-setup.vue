<!-- should generate diagnostics -->
<script setup lang="ts">
defineProps({
  user: Object,
  items: Array,
  settings: { type: Object },
  tags: { type: Array, required: true },
  value: [Object, String],
  list: { type: [String, Array] },
  wrapped: (Object),
  parenthesizedOptions: ({ type: Object }),
  type: Object,
});
</script>
