<!-- should not generate diagnostics -->
<script setup>
import window from "./browser";
const globalThis = { document: {} };
console.log(window.innerWidth, globalThis.document);
</script>
