<!-- should not generate diagnostics -->
<script setup lang="ts">
import { withDefaults } from "./utils";

const props = withDefaults(defineProps<{ name: string }>(), {
  name: "World",
});
</script>
