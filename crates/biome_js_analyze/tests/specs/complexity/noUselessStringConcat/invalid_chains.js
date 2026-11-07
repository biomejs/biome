/* should generate diagnostics */
const a = foo + 'a' + 'b'
const a = foo + `a` + `b`
const a = obj.foo + 'a' + 'b'
const a = foo + 'a' + 1
const a = foo + ('a') + 'b'
const a = (foo + 'a') + 'b'
const a = ('a') + 'b'
const a = 'a' + ('b')
const a = 1 + 2 + 'b' + 'c'
const a = foo + `${x}` + 'a' + 'b'
const a = 'a' + `${x}` + 'b' + 'c'
// Only the literals on the same line are joined
const a = foo + 'a' +
  'b' + 'c'
const a = 'a' +
  'b' + 'c'
