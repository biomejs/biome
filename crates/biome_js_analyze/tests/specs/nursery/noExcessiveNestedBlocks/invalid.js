/* should generate diagnostics */
function fiveLevels() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    if (e) {}
                }
            }
        }
    }
}

// Only the outermost statement exceeding the limit is reported.
function sevenLevels() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    if (e) {
                        if (f) {
                            if (g) {}
                        }
                    } else {
                        while (h) {}
                    }
                }
            }
        }
    }
}

// Sibling statements that exceed the limit are reported separately.
function siblings() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    for (;;) {}
                    while (e) {}
                }
            }
        }
    }
}

// An `else if` doesn't add a level, but statements inside its body do.
function elseIfBody() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                } else if (e) {
                    if (f) {}
                }
            }
        }
    }
}

// Statements at the top level of a module are counted too.
if (a) {
    if (b) {
        if (c) {
            if (d) {
                if (e) {}
            }
        }
    }
}

// A nested function starts counting from zero, and is checked on its own.
function nestedFunction() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    if (e) {
                        const inner = () => {
                            if (a) {
                                if (b) {
                                    if (c) {
                                        if (d) {
                                            if (e) {}
                                        }
                                    }
                                }
                            }
                        };
                    }
                }
            }
        }
    }
}

const arrow = () => {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    if (e) {}
                }
            }
        }
    }
};

class Foo {
    method() {
        if (a) {
            if (b) {
                if (c) {
                    if (d) {
                        if (e) {}
                    }
                }
            }
        }
    }

    static {
        if (a) {
            if (b) {
                if (c) {
                    if (d) {
                        if (e) {}
                    }
                }
            }
        }
    }
}

const object = {
    get value() {
        if (a) {
            if (b) {
                if (c) {
                    if (d) {
                        if (e) {}
                    }
                }
            }
        }
    },
};
