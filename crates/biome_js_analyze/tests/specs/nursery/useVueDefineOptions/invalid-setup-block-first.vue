<!-- should generate diagnostics -->
<script setup>
import { ref } from "vue";
const props = defineProps(["foo"]);
const count = ref(0);
</script>

<script>
export default {
  name: "Foo",
  inheritAttrs: false,
};
</script>
