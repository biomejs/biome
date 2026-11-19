<script setup lang="ts">
/* should not generate diagnostics */
import type { Props } from './types';
const { foo } = defineProps<{ foo?: string }>();
const { bar = 'default' } = defineProps<{ bar?: string }>();
const { baz } = defineProps<Props>();
defineProps<{ qux?: string }>();
const empty = defineProps<{}>();
</script>
