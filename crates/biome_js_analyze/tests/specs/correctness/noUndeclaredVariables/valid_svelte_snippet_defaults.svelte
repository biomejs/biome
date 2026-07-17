<!-- should not generate diagnostics -->
<script>
let fallback = {};
let emptyList = [];
</script>

{#snippet withArrayDefault([first, second] = emptyList)}
    <span>{first} - {second}</span>
{/snippet}

{#snippet withPlainDefault(image = fallback)}
    <img src={image.src} />
{/snippet}
