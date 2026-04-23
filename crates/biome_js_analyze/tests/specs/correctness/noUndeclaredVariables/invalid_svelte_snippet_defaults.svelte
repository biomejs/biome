<!-- should generate diagnostics -->
<script>
let fallback = {};
let emptyList = [];
</script>

{#snippet withPlainDefault(image = fallback)}
    <img src={image.src} alt={other} />
{/snippet}

{#snippet withObjectDefault({ src, caption } = fallback)}
    <img {src} alt={missing} />
{/snippet}

{#snippet withArrayDefault([first, second] = emptyList)}
    <span>{first} - {third}</span>
{/snippet}

{#snippet withNestedObjectDefault({ item: { src } = fallback })}
    <img src={other.path} alt={src} />
{/snippet}

{#snippet withNestedArrayDefault([{ id } = fallback])}
    <span>{id} - {label}</span>
{/snippet}

{#snippet withRestDefault({ src, ...rest } = fallback)}
    <img {src} alt={unknownAttr} />
{/snippet}
