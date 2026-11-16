<script setup lang="ts">
/* should not generate diagnostics */
const model = defineModel<string>({ required: true })
const count = defineModel<number>('count')
</script>
