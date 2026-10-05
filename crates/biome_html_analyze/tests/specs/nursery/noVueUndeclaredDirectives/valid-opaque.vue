<!-- should not generate diagnostics -->
<script>
const externalDirectives = {};
const sharedMixin = {};

export default {
    directives: {
        knownDirective: {},
        ...externalDirectives,
    },
    mixins: [sharedMixin],
};
</script>

<template>
    <div v-known-directive></div>
    <div v-may-come-from-a-mixin></div>
</template>
