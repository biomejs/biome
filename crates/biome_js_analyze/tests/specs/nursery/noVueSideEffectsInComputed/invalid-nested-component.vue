<!-- should generate diagnostics -->
<script setup>
import { computed, defineComponent, h, reactive } from "vue";

const Child = defineComponent({
  setup() {
    const state = reactive({ n: 0 });
    const count = computed(() => state.n++);
    return () => h("div", count.value);
  },
});
</script>
