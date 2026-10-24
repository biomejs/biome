<!-- should generate diagnostics -->
<script setup lang="ts">
import { computed } from "vue";

const otherLibrary = computed(() => someLib.string().catch(42));
const nestedName = computed(() => x.z.number().catch(42));
const promiseMethod = computed(() => Promise.resolve(z));
</script>
