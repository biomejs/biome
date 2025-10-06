// TypeScript specific test cases

// TypeScript interface props
export default {
  setup(props: { foo: string; bar: number }) {
    const { foo, bar } = props;
    return () => h('div', foo + bar)
  }
}

// Type alias props
type Props = {
  foo: string;
  bar: number;
};

export default {
  setup(props: Props) {
    const { foo, bar } = props;
    return () => h('div', foo + bar)
  }
}

// Generic component props
export default {
  setup<T extends { foo: string }>(props: T) {
    const { foo } = props;
    return () => h('div', foo)
  }
}

// defineComponent with TypeScript
export default defineComponent({
  setup(props: { foo: string }) {
    const { foo } = props;
    return () => h('div', foo)
  }
});