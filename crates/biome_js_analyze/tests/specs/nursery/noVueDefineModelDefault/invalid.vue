<script setup>
/* should generate diagnostics */
const model = defineModel({ type: String, default: '' })
const visible = defineModel('visible', { type: Boolean, default() { return false } })
const quoted = defineModel('quoted', { 'default': 1 })
const spread = defineModel('spread', { ...base, default: 1 })
defineModel('statement', { default: 'x' })
</script>
