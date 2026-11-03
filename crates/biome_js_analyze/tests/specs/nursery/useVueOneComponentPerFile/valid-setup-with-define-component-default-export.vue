<!-- should not generate diagnostics -->
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({ inheritAttrs: false });
</script>

<script setup lang="ts">
const props = defineProps<{ title: string }>();
</script>
