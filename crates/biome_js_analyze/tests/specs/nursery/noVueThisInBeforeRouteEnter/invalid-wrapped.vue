<script lang="ts">
/* should generate diagnostics */
import type { ComponentOptions } from "vue";

export default ({
  beforeRouteEnter() {
    this.method();
  },
} as ComponentOptions);
</script>
