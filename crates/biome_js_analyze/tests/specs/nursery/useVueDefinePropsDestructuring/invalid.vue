<script setup>
/* should generate diagnostics */
const props = defineProps(['foo']);
const propsWithDefaults = withDefaults(defineProps(['foo']), { foo: 'default' });
const { foo } = withDefaults(defineProps(['foo']), { foo: 'default' });
withDefaults(defineProps(['foo']), { foo: 'default' });
const [first] = defineProps(['foo']);
const parenthesized = (defineProps(['foo']));
const { bar } = withDefaults((defineProps(['bar'])), { bar: 'default' });
import { propsObject } from './props';
const fromObject = defineProps(propsObject);
let reassignable = defineProps({ foo: String });
</script>
