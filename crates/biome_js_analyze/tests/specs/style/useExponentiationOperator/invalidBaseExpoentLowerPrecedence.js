// base and exponent with a lower precedence
Math.pow(a * b, c)
Math.pow(a, b * c)
Math.pow(a / b, c)
Math.pow(a, b / c)
Math.pow(a + b, 3)
Math.pow(2, a - b)
Math.pow(a + b, c + d)
Math.pow(a = b, c = d)
Math.pow(a += b, c -= d)
Math.pow((a, b), (c, d))
function *f() { Math.pow(yield, yield) }
