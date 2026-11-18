/* should generate diagnostics */
import { defineComponent, h } from "vue";

// `$emit` of another object is not the component's own.
export const OtherObject = defineComponent({
  emits: ["foo"],
  methods: {
    setFoo() {
      custom.$emit("foo", newFoo);
    },
  },
});

export const OtherObjectInSetup = defineComponent({
  emits: ["foo"],
  setup() {
    const setFoo = () => foo.$emit("foo");
    return { setFoo };
  },
});

export const RenamedEmit = defineComponent({
  emits: ["foo", "bar"],
  setup(_, { emit: customName }) {
    const setFoo = () => customName("foo");
    return { setFoo };
  },
});

export const SetupContext = defineComponent({
  emits: ["foo", "bar"],
  setup(_, context) {
    const setFoo = () => {
      context.emit("foo", newFoo);
    };
    return { setFoo };
  },
});

export const SetupAndMethods = defineComponent({
  emits: ["foo", "bar", "baz"],
  setup(_, { emit }) {
    const emitBar = () => {
      emit("bar", newBar);
    };
    return { emitBar };
  },
  methods: {
    setFoo() {
      this.$emit("foo", newFoo);
    },
  },
});

export const UnusedContext = defineComponent({
  emits: ["foo"],
  setup(_, ctx) {
    return () => h("button");
  },
});

export const OptionalChain = defineComponent({
  emits: ["foo", "bar"],
  methods: {
    click() {
      this?.$emit?.("foo");
    },
  },
});

// A function with its own `this` does not emit on the component.
export const NestedFunction = defineComponent({
  emits: ["foo"],
  methods: {
    click() {
      return function () {
        this.$emit("foo");
      };
    },
  },
});

// The names that can be read are still checked.
export const PartlyReadable = defineComponent({
  emits: ["foo", ...others],
});
