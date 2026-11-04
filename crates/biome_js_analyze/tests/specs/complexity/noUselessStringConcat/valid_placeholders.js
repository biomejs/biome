/* should not generate diagnostics */
const a = 'a' + '${b}'
const a = '$' + '{PLUGINS}'
const a = 'a' + '${b}' + 'c'
const a = '${b}' + 'a'
const a = foo + '$' + '{b}'
const a = `$` + `{b}`
const a = '$' + `{${x}}`
const a = `${x}$` + '{y}'
