<script module lang="ts">
// Declared in the module script and referenced in neither block nor the
// template: still reported as unused.
function unusedHelper(): string {
  return "unused";
}
</script>

<script lang="ts">
const value = 1;
</script>

<p>{value}</p>
