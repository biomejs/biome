<!-- should generate diagnostics -->
<script setup lang="ts">
const a = defineEmits<{
	(e: "change", id: number): void;
	(e: "update", value: string): void;
}>();

const b = defineEmits<{
	change: [id: number];
}>();

const c = defineEmits<{ change: [id: number] }>(["change"]);
</script>
