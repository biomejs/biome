/* should generate diagnostics */
require.ensure("./entry")
require["ensure"]("./entry")
require?.ensure("./entry")

arguments.callee
arguments["callee"]

foo.__defineGetter__
foo["__defineGetter__"]

const { ensure } = require
const { callee } = arguments
const { __defineGetter__ } = foo

({ callee } = arguments)
