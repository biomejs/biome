<!-- should generate diagnostics -->
<script>
import { props } from "./stores.js";
import { state } from "./stores.js";
import { restProps } from "./stores.js";
import { slots } from "./stores.js";
</script>

<p>{$props()} {$state(0)} {$$restProps} {$$slots}</p>
