/* should not generate diagnostics */
const a = 1 + 1
const a = 1 * '2'
const a = 1 - 2
const a = foo + bar
const a = 'foo' + bar
const a = foo + 'a' + 'b'
const a = foo + `a` + `b`
const a = (number + 1) + 'px'
const a = (1 + +2) + `b`
const stylisticConcat = 'foo' + // formatting
                        'bar'
const stylisticConcat = `foo` +
                        'bar' +
                        `baz`
const stylisticConcatLeading = 'foo' // formatting
                        + 'bar'
const stylisticConcatLeading = `foo`
                        + 'bar'
                        + `baz`
