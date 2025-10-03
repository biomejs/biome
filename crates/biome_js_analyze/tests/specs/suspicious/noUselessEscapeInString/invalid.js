var s = {
    '\a': /*before*/ "\a" /*after*/,
    '\"': "\'",
    "abc\defg": ` test ${1} \a` /*after*/,
    // A test with unicode characters that take more than one byte
    key: "😀\😀",
    // https://github.com/biomejs/biome/issues/6039
    templateLiterals1: `\$x`,
    templateLiterals2: `\${\a`,
    templateLiterals3: `\${} \a`,
    // https://github.com/biomejs/biome/issues/7595
    templateLiterals4: `a\{`,
    templateLiterals5: `$\{\a`,
    templateLiterals6: `$\{} \a`,
};
