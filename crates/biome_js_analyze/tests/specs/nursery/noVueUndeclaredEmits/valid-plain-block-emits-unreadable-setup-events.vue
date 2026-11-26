<!-- should not generate diagnostics -->
<script lang="ts">
export default {
  methods: {
    notify() {
      this.$emit("change");
    },
  },
};
</script>

<script setup lang="ts">
import type { Events } from "./events";

defineEmits<Events>();
</script>
