<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
	defineProps<{
		kind: string;
		count?: number;
		enabled?: boolean;
	}>(),
	{
		count: 0,
		enabled: true,
	},
);

const label = computed(() => `${props.kind}:${props.count}`);
</script>
