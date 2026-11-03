<script setup>
/* should not generate diagnostics */
const { a = "abc", b = [], c = {}, d = () => 1 } = defineProps({
  a: { type: String },
  b: { type: Array },
  c: Object,
  d: Function,
});
</script>
