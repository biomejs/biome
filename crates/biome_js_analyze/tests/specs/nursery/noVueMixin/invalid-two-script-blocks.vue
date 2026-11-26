<!-- should generate diagnostics -->
<script>
import counterMixin from "./counterMixin";

export default {
  mixins: [counterMixin],
};
</script>

<script setup>
import { ref } from "vue";

const label = ref("Count");
</script>
