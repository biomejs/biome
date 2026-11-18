<!-- should generate diagnostics -->
<script setup>
import { shared } from "./shared";

const base = { inheritAttrs: false };
let count = 1;
const label = `Foo`;
function getName() {
  return "Foo";
}

const options = defineOptions({
  ...base,
  ...shared,
  name: getName(),
  label,
  inheritAttrs: count > 0,
});
</script>
