/* should generate diagnostics */

// method before property
class MethodBeforeProperty {
    method() {}
    name;
}

// method before constructor
class MethodBeforeConstructor {
    method() {}
    constructor() {}
}

// constructor before property
class ConstructorBeforeProperty {
    constructor() {}
    name;
}

// multiple violations: method -> constructor -> property
class MultipleViolations {
    method() {}
    constructor() {}
    name;
}
