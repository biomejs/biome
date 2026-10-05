<!-- should not generate diagnostics -->
<template>
<iframe title="This is a unique title"></iframe>
<iframe v-bind:title="titleValue"></iframe>
<iframe :title="titleValue"></iframe>
<Iframe />
</template>
