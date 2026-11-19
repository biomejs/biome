<!-- should generate diagnostics -->
<script setup lang="ts">
const a = defineEmits(["change", "update"]);

const b = defineEmits<{
	(e: "change", id: number): void;
	(e: "update", value: string): void;
}>();

const c = defineEmits<{
	change: [id: number];
	(e: "update", value: string): void;
}>();

const d = defineEmits<(e: "change", id: number) => void>();

const e = defineEmits<({ (e: "change", id: number): void })>();

const f = defineEmits<{
	change(id: number): void;
	[name: string]: unknown[];
}>();

const g = defineEmits<{ change: [id: number] }>(["change"]);
</script>
