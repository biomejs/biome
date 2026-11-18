<!-- should generate diagnostics -->
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {},
});
</script>
<script setup lang="ts">
defineProps<{ count: number }>();
</script>
