<!-- should not generate diagnostics -->
<script setup>
import { defineEmits } from "./macros";

const events = ["notify"];
defineEmits(events);
defineEmits();
</script>
