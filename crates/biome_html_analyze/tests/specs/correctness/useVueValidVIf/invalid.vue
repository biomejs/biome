<!-- should generate diagnostics -->
<template>
<div v-if:aaa="foo"></div>
<div v-if.mod="foo"></div>
<div v-if></div>
<div v-if="foo" v-else></div>
<div v-if="foo" v-else-if="bar"></div>
</template>
