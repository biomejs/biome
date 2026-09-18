/* should not generate diagnostics */

Math.log(x) * Math.log(x);
Math.LOG10E * Math.LOG10E;
Math.log(x) * Math[LOG10E];
Math.log(x) * LOG10E;
Math[log](x) * Math.LOG10E;
foo.Math.log(x) * Math.LOG10E;
Math.log(x) * foo.Math.LOG10E;
Math.log(x) * Math.NOT_LOG10E;
Math.log(x) * Math?.LOG10E;
Math?.log(x) * Math.LOG10E;
Math.log?.(x) * Math.LOG10E;
log(x) * Math.LOG10E;
new Math.log(x) * Math.LOG10E;
Math.not_log(x) + Math.LOG10E;
Math.log(x)[Math.LOG10E];
Math.log() * Math.LOG10E;
Math.log(x, extraArgument) * Math.LOG10E;
Math.log(...x) * Math.LOG10E;

Math.LN10 / Math.LN10;
Math.log(x) / Math[LN10];
Math.log(x) / LN10;
Math[log](x) / Math.LN10;
foo.Math.log(x) / Math.LN10;
Math.log(x) / foo.Math.LN10;
Math.log(x) / Math.log(x);
Math.log(x) / Math.NOT_LN10;
Math.log(x) / Math?.LN10;
Math?.log(x) / Math.LN10;
Math.log?.(x) / Math.LN10;
log(x) / Math.LN10;
new Math.log(x) / Math.LN10;
Math.not_log(x) + Math.LN10;
Math.log(x)[Math.LN10];
Math.log() / Math.LN10;
Math.log(x, extraArgument) / Math.LN10;
Math.log(...x) / Math.LN10;

Math.notSqrt(a ** 2 + b ** 2);
NotMath.sqrt(a ** 2 + b ** 2);
Math.sqrt(a ** 2 - b ** 2);
Math.sqrt(a ** 2 + 2 ** b);
Math.sqrt(a * c + b * c);
Math.sqrt(++a * ++a);
Math.sqrt(Math.pow(a, 2) + Math.pow(b, 2));
Math.sqrt(a ** 2, b ** 2);
Math.sqrt(...values);
Math.sqrt(value?.x * value.x);
Math.sqrt(value.x * value?.x);
Math.sqrt(value?.[key] * value[key]);
Math.sqrt(value[key] * value?.[key]);

function shadowed(Math) {
    Math.log(x) * Math.LOG10E;
    Math.sqrt(a ** 2 + b ** 2);
}
