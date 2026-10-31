<!-- should not generate diagnostics -->
<script setup lang="ts">
import type { Imported } from "./types";

defineProps<{ foo: string; bar?: number }>();
const typed = defineModel({ type: String });
const namedTyped = defineModel("namedTyped", { type: String });
const constructor = defineModel(String);
const namedConstructor = defineModel("namedConstructor", Number);
const generic = defineModel<string>();
const namedGeneric = defineModel<Imported>("namedGeneric");
const validated = defineModel("validated", { validator: (value) => value > 0 });
</script>
