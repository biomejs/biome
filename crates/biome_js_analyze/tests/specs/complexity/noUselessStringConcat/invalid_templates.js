/* should generate diagnostics */
const a = `a${x}` + `b`
const a = 'a' + `b${x}`
const a = `a${x}` + `b${y}`
const a = `a` + `b${x}`
const a = `a${x}` + ''
const a = "a`b" + `${x}`
const a = 'a${b}' + `${x}`
const a = 'a\'b' + `${x}`
const a = '\0' + `${x}`
const a = 'a' + `b${x}` + 'c'
const a = foo + `a${x}` + 'b'
function foo() { return `a` + `b${x}` }
function foo() { throw `a` + `b${x}` }
const fn = () => `a` + `b${x}`
const a = 'a' + 'b' + `${x}`
// The fix would change the escape sequence `\0` into `\01`
const a = '\0' + `1${x}`
const a = `${x}\0` + '1'
// The fix would call `bar()` before converting `foo` to a string
const a = foo + 'a' + `${bar()}`
const a = foo + `a` + `b${baz()}`
// The fix would remove the parentheses
const a = (foo + 'a') + `b${x}`
// The fix would turn `foo` into a template tag
foo
'a' + `b${x}`
// A string literal can't contain a line break
const a = foo + `a
b` + 'c'
