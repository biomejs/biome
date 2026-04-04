<!-- should not generate diagnostics -->
<script>
import { Component } from "./component.svelte";
let hello = "Hello World";
let array = [];
let props = [];
</script>

<html>
    <span>{hello}</span>
    <span>{notDefined}</span>
    {#each array as item}
    {/each}
    <Component />
    <input {...props} />
</html>
