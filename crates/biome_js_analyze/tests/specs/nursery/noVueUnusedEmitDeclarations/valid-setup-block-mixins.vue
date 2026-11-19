<!-- should not generate diagnostics -->
<script>
export default {
  emits: ["foo"],
};
</script>

<script setup>
import emitsFoo from "./emitsFoo";

defineOptions({ mixins: [emitsFoo] });
</script>
