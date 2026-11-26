/* should not generate diagnostics */
function fourLevels() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {}
            }
        }
    }
}

// An `else if` doesn't add a level of nesting.
function elseIfChain() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                } else if (e) {
                } else if (f) {
                } else {
                }
            }
        }
    }
}

// Plain blocks, labels, and catch clauses don't add a level of nesting.
function plainBlocks() {
    if (a) {
        if (b) {
            if (c) {
                label: {
                    {
                        try {
                        } catch {
                            {}
                        }
                    }
                }
            }
        }
    }
}

// Each function starts counting from zero.
function nestedFunctions() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    const inner = function () {
                        if (a) {
                            if (b) {
                                if (c) {
                                    if (d) {}
                                }
                            }
                        }
                    };
                }
            }
        }
    }
}

class Foo {
    constructor() {
        if (a) {
            if (b) {
                if (c) {
                    if (d) {}
                }
            }
        }
    }

    static {
        if (a) {
            if (b) {
                if (c) {
                    if (d) {}
                }
            }
        }
    }
}
