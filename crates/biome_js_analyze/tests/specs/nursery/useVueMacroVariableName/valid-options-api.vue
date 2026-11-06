<script>
/* should not generate diagnostics */
import { useAttrs } from "vue";

export default {
  setup() {
    const attributes = useAttrs();
    return { attributes };
  },
};
</script>
