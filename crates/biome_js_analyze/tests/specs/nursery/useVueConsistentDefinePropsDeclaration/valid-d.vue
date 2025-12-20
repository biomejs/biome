<script setup lang="ts">
const props = defineProps({
	kind: { type: String },
	enabled: { type: Boolean },
});

const emit = defineEmits<{
	(e: "update", value: string): void;
}>();

foo("someParam");
foo<string>();
</script>
