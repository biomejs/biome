<!-- should not generate diagnostics -->
<script lang="ts">
function handleClick() {
    console.log("hey");
}
</script>

<button type="button" onclick={handleClick}>greeting</button>
