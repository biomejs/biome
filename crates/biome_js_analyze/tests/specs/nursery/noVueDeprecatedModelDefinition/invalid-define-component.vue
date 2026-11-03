<script>
/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  model: {
    prop: "my-value",
    event: "input",
  },
});
</script>
