<!-- should not generate diagnostics -->
<script setup>
const { other = "Hello" } = defineProps({
  name: {
    required: false,
    default: "Hello",
  },
  age: {
    required: true,
  },
  other: {
    required: false,
  },
  list: ["name"],
});
</script>
