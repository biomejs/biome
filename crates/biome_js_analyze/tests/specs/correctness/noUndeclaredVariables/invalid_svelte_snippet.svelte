<!-- should generate diagnostics -->
<script>
let images = [];
</script>

{#snippet figure(image2)}
    <figure>
        <img src={image.src} />
    </figure>
{/snippet}

{#snippet destructured({ src2, caption2 })}
    <img {src} alt={caption} />
{/snippet}

{#each images as img2}
    {@render figure(img)}
{/each}
