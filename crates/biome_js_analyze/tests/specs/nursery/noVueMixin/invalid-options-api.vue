<!-- should generate diagnostics -->
<script>
import counterMixin from "./counterMixin";
import loggerMixin from "./loggerMixin";

export default {
  name: "Counter",
  mixins: [counterMixin, loggerMixin],
  data() {
    return { label: "Count" };
  },
};
</script>
