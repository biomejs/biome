<!-- should not generate diagnostics -->
<script setup>
const props = defineProps(['foo', 'bar'])
const foo = props.foo
const { bar } = props
</script>
