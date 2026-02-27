const chainedOnNewLine = (l) =>
  l
    .split('\n')

const multipleChainedMethods = (arr) =>
  arr
    .filter(x => x > 0)
    .map(x => x * 2)

const objectWithMultipleProperties = () =>
  ({
    prop1: 'value1',
    prop2: 'value2'
  })

const arrayWithMultipleElements = () =>
  [
    1,
    2,
    3
  ]

const nested = () =>
  someFunction(
    arg1,
    arg2
  )

const withInlineComment = (l) =>
  l
    /* keep this comment */
    .split('\n')

const withLineComment = (arr) =>
  arr
    // this comment is important
    .filter(x => x > 0)

const withCommentInObject = () =>
  ({
    // important note
    prop1: 'value1',
    prop2: 'value2'
  })

const withCommentInArray = () =>
  [
    // first element
    1,
    2,
    3
  ]
