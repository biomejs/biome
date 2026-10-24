<!-- should not generate diagnostics -->
<script lang="ts">
import type { PropType } from "vue";

export default {
  props: {
    names: ["a", "b"],
    user: Object as PropType<{ name: string }>,
    settings: { type: Object as PropType<Record<string, string>> },
    label: { type: String, required: true },
  },
};
</script>
