var s = {
    '\a': /*before*/ "\a" /*after*/,
    '\"': "\'",
    "abc\defg": ` test ${1} \a` /*after*/,
    // A test with unicode characters that take more than one byte
    key: "😀\😀"
};
