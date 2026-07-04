<!-- should not generate diagnostics -->
<script module lang="ts">
// `<script module>` and `<script>` compile to one module and share a
// top-level scope. These bindings are declared here but used only in the
// instance script below, so they must not be reported as unused.
function formatId(id: number): string {
  return `row-${id}`;
}

const PREFIX = "row";
</script>

<script lang="ts">
const id = formatId(1);
const label = `${PREFIX}-${id}`;
</script>

<div data-id={id}>{label}</div>
