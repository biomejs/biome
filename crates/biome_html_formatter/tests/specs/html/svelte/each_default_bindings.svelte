<script>
const fallback=0;
const rows=[];
const pairs=[];
</script>

{#each rows as {id=fallback,label:title="untitled",options={active:true}}}
<p>{id}: {title}: {options.active}</p>
{/each}

{#each pairs as [first=fallback,second=[fallback]]}
<p>{first}: {second}</p>
{/each}
