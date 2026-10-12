<!-- should not generate diagnostics -->
<script>
import emitsFoo from "./emitsFoo";

export default {
  mixins: [emitsFoo],
};
</script>

<script setup>
defineEmits(["foo"]);
</script>
