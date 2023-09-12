// unary expressions are exception by the language - parens are required for the base to disambiguate operator precedence
Math.pow(+a, b)
Math.pow(a, +b)
Math.pow(-a, b)
Math.pow(a, -b)
Math.pow(-2, 3)
Math.pow(2, -3)
async () => Math.pow(await a, b)
async () => Math.pow(a, await b)
