<!-- should not generate diagnostics -->
<script setup lang="ts">
import { onMounted } from "vue";
import { title } from "./invalid-imported.js";
import "./valid-reexport.js";
import type { Theme } from "./valid-type-imported.ts";
onMounted(() => import("./valid-dynamic-imported.js"));
</script>
