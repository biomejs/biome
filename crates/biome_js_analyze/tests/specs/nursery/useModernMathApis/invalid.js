/* should generate diagnostics */

Math.log(x) * Math.LOG10E;
Math.LOG10E * Math.log(x);
Math.log(x) / Math.LN10;
Math.log(x) * Math.LOG2E;
Math.LOG2E * Math.log(x);
Math.log(x) / Math.LN2;
Math.log(((0, x))) * Math.LOG10E;
Math.LOG10E * Math.log(((0, x)));
Math.log(((0, x))) / Math.LN10;

function logarithm(x) {
    return Math.log(x) / Math.LN10;
}

Math.sqrt(a * a + b * b);
Math.sqrt(a ** 2 + b ** 2);
Math.sqrt(a * a + b ** 2);
Math.sqrt(a * a + b * b + c * c);
Math.sqrt(a ** 2 + b ** 2 + c ** 2);
Math.sqrt(a * a);
Math.sqrt(a ** 2);
Math.sqrt(a * a,);
Math.sqrt(a ** 2,);
Math.sqrt((a, b) ** 2);
Math.sqrt((++a) ** 2);
Math.sqrt(a * a + b * b,);
Math.sqrt(a ** 2 + b ** 2,);
Math.sqrt(((a ** 2)) + ((b ** 2 + c ** 2)) + ((d)) * ((d)) + ((e)) ** ((2)));
Math.sqrt(0x2 * 2);
Math.sqrt("x" * 'x');

class Vector {
    #x;

    magnitude() {
        return Math.sqrt(this.#x * this.#x);
    }
}

/* leading logarithm comment */ Math.log(x) * Math.LOG10E; /* trailing logarithm comment */
/* leading square root comment */ Math.sqrt(a * a + b * b); /* trailing square root comment */
