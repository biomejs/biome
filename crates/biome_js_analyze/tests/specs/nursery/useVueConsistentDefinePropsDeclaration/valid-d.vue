<script setup lang="ts">
import { ref, computed } from "vue";

interface LocalState {
	count: number;
}

const props = defineProps({
	kind: { type: String },
	enabled: { type: Boolean },
});

const emit = defineEmits<{
	(e: "update", value: string): void;
}>();

const state = ref<LocalState>({ count: 0 });

const label = computed(() => {
	return props.enabled ? props.kind.toUpperCase() : "disabled";
});

// unrelated logic
function increment() {
	state.value.count++;
}

increment();
</script>
