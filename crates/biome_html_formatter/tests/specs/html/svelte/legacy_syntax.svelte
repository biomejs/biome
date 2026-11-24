<script>export let count=0; $: doubled=count*2;</script>
<button on:click = {() => count++}>{doubled}</button>
<slot name = "label" />
