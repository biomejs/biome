<script setup lang="ts">
/* should not generate diagnostics */
import { defaults } from "./defaults";

withDefaults(defineProps<{ foo?: string }>(), defaults);
</script>
