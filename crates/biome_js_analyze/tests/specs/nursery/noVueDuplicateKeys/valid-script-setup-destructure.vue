<script setup>
const { foo, bar } = defineProps(['foo', 'bar'])

const localVar = ref('local')
const computedValue = computed(() => foo + bar)
</script>