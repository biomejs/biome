<script setup lang="ts">
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
</script>
