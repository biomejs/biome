<script setup>
/* should not generate diagnostics */
import { defineEmits } from "./my-macros";

const emit = defineEmits(["click"]);
emit("click");
</script>
