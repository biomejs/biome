/* should generate diagnostics */
const selected = new Set();
const other = new Set();

previous()
new Set(condition ? selected : other).union(other)

previous()
new Set((condition ? selected : other).union(other))

const functionExpression = function () {}
new Set(condition ? selected : other).union(other)

const classExpression = class {}
new Set(condition ? selected : other).union(other)

const arrowFunction = () => {}
new Set(condition ? selected : other).union(other)

function functionDeclaration() {}
new Set(condition ? selected : other).union(other)

class ClassDeclaration {}
new Set(condition ? selected : other).union(other)

const functionExpressionBeforeTrue = function() {}
new Set(true ? selected : other).union(other)

const classExpressionBeforeTrue = class {}
new Set(true ? selected : other).union(other)

previous()
new Set(selected.union(other))
