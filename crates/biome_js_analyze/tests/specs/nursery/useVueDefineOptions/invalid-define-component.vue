<!-- should generate diagnostics -->
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Foo",
});
</script>

<script setup lang="ts">
const props = defineProps<{ foo: string }>();
</script>
