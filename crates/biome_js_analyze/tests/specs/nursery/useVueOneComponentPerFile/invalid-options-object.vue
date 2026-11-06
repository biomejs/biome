<!-- should generate diagnostics -->
<script>
import { createApp, defineComponent } from "vue";

const TodoItem = defineComponent({ props: ["title"] });
createApp({ name: "Root" });

export default {
  components: { TodoItem },
};
</script>
