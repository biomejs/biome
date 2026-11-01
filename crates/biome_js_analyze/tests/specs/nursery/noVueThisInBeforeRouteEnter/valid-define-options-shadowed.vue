<script setup>
/* should not generate diagnostics */
import { defineOptions } from "./local-helpers";

defineOptions({
  beforeRouteEnter() {
    this.load();
  },
});
</script>
