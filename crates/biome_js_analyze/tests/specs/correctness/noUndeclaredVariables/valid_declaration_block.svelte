<!-- should not generate diagnostics -->
<script>
let boxes = [];
</script>

{#each boxes as box}
    {let area = box.width * box.height}
    {const { width, height } = box}
    {const [first, second] = box.points}
    <span>{area} {width} {height} {first} {second}</span>
{/each}
