// Parameter destructuring patterns that lose reactivity

// Basic parameter destructuring
export default {
  setup({ foo, bar }) {
    return () => h('div', foo + bar)
  }
}

// Destructuring with default values  
export default {
  setup({ foo = 'default', bar }) {
    return () => h('div', foo + bar)
  }
}

// Destructuring with renaming
export default {
  setup({ foo: renamedFoo, bar }) {
    return () => h('div', renamedFoo + bar)
  }
}

// Destructuring with rest pattern
export default {
  setup({ foo, ...rest }) {
    return () => h('div', foo + rest.bar)
  }
}

// defineComponent with parameter destructuring
export default defineComponent({
  setup({ foo, bar }) {
    return () => h('div', foo + bar)
  }
})

// Named export with parameter destructuring  
export const MyComponent = {
  setup({ foo, bar }) {
    return () => h('div', foo + bar)
  }
}