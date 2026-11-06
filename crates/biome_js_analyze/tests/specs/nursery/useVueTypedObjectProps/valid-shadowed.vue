<!-- should not generate diagnostics -->
<script setup lang="ts">
import { Object, Array } from "./types";

defineProps({
  user: Object,
  items: { type: Array },
});
</script>
