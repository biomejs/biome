<!-- should not generate diagnostics -->
<script setup>
import { defineEmits } from "./my-macros";

const emit = defineEmits(["click"]);
emit("click");
</script>
