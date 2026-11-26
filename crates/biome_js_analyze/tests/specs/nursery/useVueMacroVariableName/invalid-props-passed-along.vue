<script setup>
/* should generate diagnostics */
import { toRef } from "vue";

const properties = defineProps(["foo"]);
const foo = toRef(properties, "foo");
</script>
