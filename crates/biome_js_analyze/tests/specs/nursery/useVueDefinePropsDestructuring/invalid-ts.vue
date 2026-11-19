<script setup lang="ts">
/* should generate diagnostics */
import type { Props } from './types';
const props = defineProps<{ foo?: string }>();
const imported = defineProps<Props>();
const propsWithDefaults = withDefaults(defineProps<{ foo?: string }>(), { foo: 'default' });
const { foo } = withDefaults(defineProps<{ foo?: string }>(), { foo: 'default' });
withDefaults(defineProps<{ foo?: string }>(), { foo: 'default' });
const asserted = defineProps<{ foo?: string }>() as { foo?: string };
</script>
