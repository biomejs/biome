function booleanOperators2() {
    if (a                                     // +1 for `if`
        &&                                    // +1
        !(b && c)) {                          // +1
        return true;
    }
}
