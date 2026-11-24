<script lang="ts">
let count:number=$state(1)
const label=$derived(`count: ${count}`)
</script>

<p>{count}ms: <strong>{label}</strong> <em>now</em>!</p>
<p>+{count}<span>ms</span><span>!</span></p>
<p>Elapsed: <strong aria-label="elapsed time in milliseconds">{count}ms</strong> <em>{label}</em>!</p>
<pre>  {label}
    {count}  ms</pre>
<textarea>  {label}
    {count}  ms</textarea>

<style>
p strong::before{content:"  =  "}
</style>
