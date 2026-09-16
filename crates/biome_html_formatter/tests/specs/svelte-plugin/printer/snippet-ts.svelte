<script lang="ts">
</script>

{#snippet test1(arg: string)}
    foo
{/snippet}

{#snippet test2(arg: (typeof obj)["key"])}
    foo
{/snippet}

{#snippet test3({ onselect }: { onselect: (value: string) => void })}
    foo
{/snippet}
