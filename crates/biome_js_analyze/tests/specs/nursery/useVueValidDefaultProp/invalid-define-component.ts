/* should generate diagnostics */
import { defineComponent, type PropType } from "vue";

export default defineComponent({
  props: {
    a: { type: Array as PropType<string[]>, default: [] },
    b: { type: Object as PropType<{ [key: number]: number }>, default: {} },
    c: { type: Function as PropType<() => number>, default: 10 },
    d: { type: [Object, Number], default: {} } as PropOptions<object>,
  },
});
