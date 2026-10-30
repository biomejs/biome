<!-- should generate diagnostics -->
<script setup>
const props = defineProps({
  name: {
    required: true,
    default: "Hello",
  },
  list: {
    type: Array,
    required: true,
    default() {
      return [];
    },
  },
});
</script>
