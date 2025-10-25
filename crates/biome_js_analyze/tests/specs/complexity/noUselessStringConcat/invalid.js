const a = /*1*/ 'a' + 'b' // comment
const a = 'a' + 'b' + 'c'
const a = 'a' + ('b' + 'c')
const a = ('a' + 'b') + 'c'
const a = foo + 'a' + 'b' + 'c'
const a = (foo + 'a') + ('b' + 'c')
const a = ((foo + 'a') + ('b' + 'c') + 1)
const a = 'a' + `b`
const a = `a` + 'b'
const a = `a` + `b`
const a = 'a' + 1
const a = 1 + '1'
const a = 1 + `1`
const a = `1` + 1
const a = 1 + 1 + ""
const multilineBefore = 1
    + 1 // comment
    + "bar"
const multilineAfter = 12 +
    2 +
    "foo"
