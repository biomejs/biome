<script setup>
/* should generate diagnostics */
import { computed, ref } from "vue";

const count = ref(0);
const empty = computed(() => {});
const functionExpression = computed(function () {});
const noElse = computed(() => {
  if (count.value > 0) {
    return count.value;
  }
});
const emptyReturn = computed(() => {
  return;
});
const accessors = computed({
  set: (value) => {
    count.value = value;
  },
  get: () => {},
});
const methodAccessor = computed({
  get() {
    count.value * 2;
  },
});
const innerFunctionReturns = computed(() => {
  const double = () => {
    return count.value * 2;
  };
  double();
});
</script>
