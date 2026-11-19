<!-- should generate diagnostics -->
<template>

<!-- argument not allowed -->
<div v-once:arg></div>

<!-- modifier not allowed -->
<div v-once.mod></div>

<!-- value not allowed -->
<div v-once="value"></div>
</template>
