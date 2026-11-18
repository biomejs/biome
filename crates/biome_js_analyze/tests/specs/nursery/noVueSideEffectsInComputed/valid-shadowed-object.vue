<!-- should not generate diagnostics -->
<script setup>
import { computed, reactive } from "vue";

const Object = { assign: (target) => ({ ...target }) };
const user = reactive({ name: "" });
const copy = computed(() => Object.assign(user, { name: "lorem" }));
</script>
