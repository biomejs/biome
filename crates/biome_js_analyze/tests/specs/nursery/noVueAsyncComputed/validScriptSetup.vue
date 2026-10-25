<!-- should not generate diagnostics -->
<script setup>
import { computed, ref } from "vue";

const data = ref(null);
const loaded = computed(() => data.value !== null);

async function load() {
	data.value = await fetchData();
}

setTimeout(load, 0);
</script>
<script>
export default {
	name: "App",
};
</script>
