<!-- should not generate diagnostics -->
<script lang="ts">
    import { Fallback } from './items';
    const items: { value?: Fallback } = {};
</script>
{#await items then { value = new Fallback() }}{value}{/await}
