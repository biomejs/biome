// Variable destructuring patterns that lose reactivity

// const destructuring
export default {
  setup(props) {
    const { foo, bar } = props;
    return () => h('div', foo + bar)
  }
}

// let destructuring
export default {
  setup(props) {
    let { foo, bar } = props;
    return () => h('div', foo + bar)
  }
}

// var destructuring  
export default {
  setup(props) {
    var { foo, bar } = props;
    return () => h('div', foo + bar)
  }
}

// Assignment destructuring
export default {
  setup(props) {
    let foo, bar;
    ({ foo, bar } = props);
    return () => h('div', foo + bar)
  }
}

// Deep destructuring
export default {
  setup(props) {
    const { user: { name, age } } = props;
    return () => h('div', name + age)
  }
}

// Array destructuring from props property
export default {
  setup(props) {
    const [first, second] = props.items;
    return () => h('div', first + second)
  }
}