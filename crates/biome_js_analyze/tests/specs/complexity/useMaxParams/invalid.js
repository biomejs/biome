function tooManyParams(a, b, c, d, e, f, g, h) {
    return a + b + c + d + e + f + g + h;
}

function namedFunction(a, b, c, d, e, f, g, h, i) {
    return a + b + c + d + e + f + g + h + i;
}

const fn1 = function(a, b, c, d, e, f, g, h) {
    return a + b + c + d + e + f + g + h;
};

const fn2 = function namedFnExpression(a, b, c, d, e, f, g, h, i) {
    return a + b + c + d + e + f + g + h + i;
};

const arrow1 = (a, b, c, d, e, f, g, h) => {
    return a + b + c + d + e + f + g + h;
};

const arrow2 = (a, b, c, d, e, f, g, h, i) => a + b + c + d + e + f + g + h + i;

class MyClass {
    method(a, b, c, d, e, f, g, h) {
        return a + b + c + d + e + f + g + h;
    }

    constructor(a, b, c, d, e, f, g, h, i) {
        this.sum = a + b + c + d + e + f + g + h + i;
    }
}

const obj = {
    method(a, b, c, d, e, f, g, h) {
        return a + b + c + d + e + f + g + h;
    }
};

function withThisParam(this, a, b, c, d, e, f, g) {
    return a + b + c + d + e + f + g;
}
