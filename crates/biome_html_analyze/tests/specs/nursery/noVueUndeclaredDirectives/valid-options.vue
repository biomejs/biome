<!-- should not generate diagnostics -->
<script>
export default {
    directives: {
        "kebab-directive": {},
        camelDirective: {},
        PascalDirective: {},
        "quoted-directive": {},
    },
};
</script>

<template>
    <div v-kebab-directive></div>
    <div v-camel-directive></div>
    <div v-pascal-directive></div>
    <div v-quoted-directive></div>
</template>
