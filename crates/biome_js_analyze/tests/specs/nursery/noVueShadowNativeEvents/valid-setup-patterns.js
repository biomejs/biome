/* should not generate diagnostics */
import { defineComponent } from "vue";

export const RestContext = defineComponent({
  setup(props, ...context) {
    context.emit("click");
  },
});

export const ArrayContext = defineComponent({
  setup(props, [emit]) {
    emit("click");
  },
});

export const DollarEmit = defineComponent({
  setup(props, { $emit }) {
    $emit("click");
  },
});

export const OnlyProps = defineComponent({
  setup(context) {
    context.emit("click");
  },
});

export const ImmediatelyInvoked = defineComponent({
  setup: ((props, context) => {
    context.emit("click");
  })(),
});
