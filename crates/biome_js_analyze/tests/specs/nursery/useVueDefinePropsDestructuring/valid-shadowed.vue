<script setup>
/* should not generate diagnostics */
import { defineProps, withDefaults } from './macros';
const props = defineProps(['foo']);
const { foo } = withDefaults(defineProps(['foo']), { foo: 'default' });
</script>
