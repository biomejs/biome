function booleanOperators() {
    if (a                                     // +1 for `if`
        && b && c                             // +1
        || d || e                             // +1
        && f) {                               // +1
        return true;
    }
}
