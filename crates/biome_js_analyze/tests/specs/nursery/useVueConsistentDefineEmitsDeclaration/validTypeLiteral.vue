<!-- should not generate diagnostics -->
<script setup lang="ts">
type Emits = { (e: "change", id: number): void };

const a = defineEmits<{
	change: [id: number];
	"update:value": [value: string];
}>();

const b = defineEmits<Emits>();

const c = defineEmits();
</script>
