<!-- should not generate diagnostics -->
<script setup lang="ts">
const a = defineEmits<{
	(e: "change", id: number): void;
}>();

const b = defineEmits<{
	change: [id: number];
}>();
</script>
