<script>
/* should not generate diagnostics */
export default {
  data() {
    return {
      copy: this.doubled,
    };
  },
};
</script>

<script setup>
import { computed } from "vue";

const doubled = computed(() => 2);
</script>
