var s = {
    '\a': "\a",
    '\"': "\'",
    "abc\defg": ` test ${1} \a`,
    // A test with unicode characters that take more than one byte
    key: "😀\😀"
};
