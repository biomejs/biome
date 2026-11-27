<!-- should generate diagnostics -->
<script>
import { defineComponent } from "vue";

export const Helper = defineComponent({ name: "Helper" });
</script>

<script setup>
const props = defineProps(["title"]);
</script>
