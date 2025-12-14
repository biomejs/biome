const returnsSequenceArrow = () => (a, b)

const returnsAwaitArrow = async () => await fetchData()

// Issue #8179: multiline expressions should not cause ASI issues
const getSchemaRowTypes = (l) =>
  l
    .split("\n")

const multilineChain = () =>
  foo
    .bar()
    .baz()
