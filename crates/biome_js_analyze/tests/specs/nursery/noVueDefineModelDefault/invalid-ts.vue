<script setup lang="ts">
/* should generate diagnostics */
const modelValue = defineModel<string>({ default: '' })
const count = defineModel<number>('count', { default: 0 })
const items = defineModel<string[]>('items', { default: () => [] })
const asConst = defineModel('asConst', { default: 'foo' } as const)
const satisfied = defineModel('satisfied', { default: 'foo' } satisfies object)
</script>
