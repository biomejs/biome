// Conditional and control flow destructuring that loses reactivity

// Conditional destructuring
export default {
  setup(props) {
    if (props.show) {
      const { foo } = props;
      return () => h('div', foo)
    }
    return () => h('div', 'hidden')
  }
}

// Destructuring in try-catch
export default {
  setup(props) {
    try {
      const { foo, bar } = props;
      return () => h('div', foo + bar)
    } catch (e) {
      return () => h('div', 'error')
    }
  }
}

// Destructuring in switch statement
export default {
  setup(props) {
    switch (props.type) {
      case 'type1': {
        const { foo } = props;
        return () => h('div', foo)
      }
      default:
        return () => h('div', 'default')
    }
  }
}

// Destructuring in for loop
export default {
  setup(props) {
    for (let i = 0; i < 1; i++) {
      const { foo } = props;
      console.log(foo);
    }
    return () => h('div', props.bar)
  }
}