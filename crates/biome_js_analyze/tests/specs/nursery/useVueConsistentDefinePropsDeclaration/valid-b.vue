<script setup lang="ts">
const props = defineProps<{
	kind: string;
	enabled?: boolean;
}>();

const emit = defineEmits<{
	(e: "update", value: string): void;
}>();

foo("someParam");
foo<string>();
</script>
