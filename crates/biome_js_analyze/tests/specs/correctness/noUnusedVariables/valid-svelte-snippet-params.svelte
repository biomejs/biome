<!-- should not generate diagnostics -->
<script>
let images = [];
</script>

{#snippet figure(image)}
    <figure>
        <img src={image.src} alt={image.caption} width={image.width} height={image.height} />
        <figcaption>{image.caption}</figcaption>
    </figure>
{/snippet}

{#each images as img}
    {@render figure(img)}
{/each}
