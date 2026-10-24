<!-- should not generate diagnostics -->
<script setup lang="ts">
const emit = defineEmits<{
	(e: "notify"): void;
	(e: "submit", id: number): void;
}>();
</script>
