<!-- should not generate diagnostics -->
<script setup lang="ts" generic="TData extends RowData">
import type { RowData } from "./types";

defineProps<{ rows: TData[] }>();
</script>
