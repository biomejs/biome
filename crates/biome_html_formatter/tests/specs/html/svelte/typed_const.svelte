<script lang="ts">
const items = [1, 2, 3];
</script>

{#each items as item}
    {@const doubled  :  number=item*2}
    {@const cells:Record<string,typeof item>={value:item}}
    {@const {value:renamed,...rest}: {value:number} = cells}
    {@const [first,...others]:number[]=[item]}
    {@const /*a*/ commented /*b*/ : /*c*/ number /*d*/ = item /*e*/}
    <span>{doubled}</span>
{/each}
