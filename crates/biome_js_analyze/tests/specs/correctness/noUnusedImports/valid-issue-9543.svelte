<!-- should not generate diagnostics -->
<script>
import { Thing as Something, Thing } from "./issue-9543-reexport";
</script>

<Something />
<Thing />
