/* should not generate diagnostics */
const props = defineProps(['foo']);
withDefaults(defineProps(['foo']), { foo: 'default' });
