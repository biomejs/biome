/* should generate diagnostics */
// Template literals don't allow these escapes, so there is no fix
var a = '\1' + `${x}`
var a = `${x}` + '\012'
var a = '\8' + `${x}`
