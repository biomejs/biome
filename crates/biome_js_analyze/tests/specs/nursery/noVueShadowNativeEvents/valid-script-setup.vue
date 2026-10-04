<script setup lang="ts">
/* should not generate diagnostics */
import type { ImportedEmits } from "./types";

const emit = defineEmits<{ save: []; "update:modelValue": [value: string] }>();
emit("save");
emit("update:modelValue", "value");

defineEmits<(e: "save" | "remove") => void>();
defineEmits<{ (e: "save"): void; (e: string): void }>();
defineEmits<ImportedEmits>();
defineEmits(unknown);
defineEmits(["save", ...others]);

function notEmit(name: string) {}
notEmit("click");
$emit("click");
</script>
