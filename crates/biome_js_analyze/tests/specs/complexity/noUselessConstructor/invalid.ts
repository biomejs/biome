/* should generate diagnostics */
class A {
    constructor() {}
}

class B extends A {
    constructor() {
        super();
    }
}

class ProtectedBase {
    protected constructor() {}
}

class Public extends ProtectedBase {
    constructor() {
        super();
    }
}
