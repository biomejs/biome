<!-- should not generate diagnostics -->
<script setup lang="ts">
import { ref, type Ref } from "vue";

const count: Ref<number | null> = ref(null);
const total = ref<number>();
</script>
