<!-- should not generate diagnostics -->
<script>
import { Component } from "./component.vue";
let hello = "Hello World";
</script>

<script>
let greeting = "Hello World";
function reasonText() {
	return "foo"
}
</script>


<template>
    <span>{hello}</span>
    <span>{notDefined}</span>
    <span>{greeting}</span>
    <Component />
    <span class="dc-reason">{{ reasonText() }}</span>
</template>
