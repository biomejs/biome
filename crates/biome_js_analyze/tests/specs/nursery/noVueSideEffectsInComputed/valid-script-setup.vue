<!-- should not generate diagnostics -->
<script setup>
import { computed, ref, watch } from "vue";
import { store } from "./store";

const count = ref(0);
const items = ref([]);

const doubled = computed(() => count.value * 2);
const reversed = computed(() => items.value.slice().reverse());
const fromStore = computed(() => store.items.reverse());

watch(count, () => {
  items.value.push(count.value);
});

function increment() {
  count.value++;
}
</script>
