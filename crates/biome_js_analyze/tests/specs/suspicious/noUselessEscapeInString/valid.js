/* should not generate diagnostics */
var s = {
    '\0\'': "\n\"",
    "abc\u42efg": tagged` test ${1} \a`,
    key: `\``,
    escapeTemplateLiteralInterpolation1: `\${`,
    escapeTemplateLiteralInterpolation2: `\${}`
};
