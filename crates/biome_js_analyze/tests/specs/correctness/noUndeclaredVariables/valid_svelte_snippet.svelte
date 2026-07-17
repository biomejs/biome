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

{#snippet destructured({ src, caption })}
    <img {src} alt={caption} />
{/snippet}

{#each images as img}
    {@render figure(img)}
{/each}
