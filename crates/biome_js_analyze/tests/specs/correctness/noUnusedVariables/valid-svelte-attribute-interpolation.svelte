<!-- should not generate diagnostics -->
<script lang="ts">
let top = 0;
let cls = "active";
let open = true;
</script>

<div style="top: {top}px" class="card {cls}"></div>
<dialog bind:open></dialog>
