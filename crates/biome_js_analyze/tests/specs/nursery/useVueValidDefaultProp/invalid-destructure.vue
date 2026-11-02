<script setup>
/* should generate diagnostics */
const {
  a = 123,
  b = [],
  c = 42,
  d = () => [],
  e = function () {
    return "e";
  },
} = defineProps({
  a: String,
  b: { type: Number },
  c: { type: Array },
  d: { type: Array },
  e: { type: String },
});
</script>
