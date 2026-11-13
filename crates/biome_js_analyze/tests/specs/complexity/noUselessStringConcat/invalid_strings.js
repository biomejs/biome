/* should generate diagnostics */
const a = 'a' + 'b\'c'
const a = '\\' + 'n'
const a = 'a' + '${}'
const a = `a` + `${b}`
const a = "a`b" + "c"
const a = <div>{'a' + 'b'}</div>
const a = '\\0' + '1'
// The fix would change the escape sequence `\0` into `\01`
const a = '\0' + '1'
const a = foo + '\0' + '1'
