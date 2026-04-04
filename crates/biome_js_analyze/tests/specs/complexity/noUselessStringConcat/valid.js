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
// Tagged templates should not be treated as plain strings
const a = t`translate-me` + "!"
const a = "prefix" + sql`query`
const a = tag`a` + tag`b`
