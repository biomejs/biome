<!-- should generate diagnostics -->
<template>
<a href="javascript:void(0)">invalid</a>
<a href>invalid</a>
<a @click="count++">invalid</a>
<a>invalid</a>
<!-- static Vue href bindings with invalid values should also be flagged -->
<a :href="'javascript:void(0)'">invalid</a>
<a v-bind:href="'javascript:'">invalid</a>
<a v-bind="spread" href="#">invalid</a>
<a v-bind="spread" href>invalid</a>
<a v-bind="spread" @click="count++">invalid</a>
<a v-bind="spread" :href="'javascript:void(0)'">invalid</a>
</template>
