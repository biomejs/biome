<!-- should generate diagnostics -->
<script setup>
const sizes = ["small", "large"];
const initial = "small";
const props = defineProps({
  size: {
    type: String,
    default: initial,
    validator: (value) => sizes.includes(value),
  },
});
</script>
