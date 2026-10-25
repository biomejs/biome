<script lang="ts">
/* should not generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  emits: {
    foo: (payload: string | number) => true,
    bar(payload: string | number) {
      return true;
    },
  },
});
</script>
