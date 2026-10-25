<script setup>
/* should generate diagnostics */
import { useAttrs, useSlots } from "vue";

const customName = defineProps({ msg: String });
const emitsWrong = defineEmits(["change"]);
const slotsWrong = defineSlots();
const useSlotsWrong = useSlots();
const attrsWrong = useAttrs();

function update() {
  emitsWrong("change", customName.msg);
  console.log(slotsWrong.default, useSlotsWrong.header, attrsWrong.class);
}
</script>
