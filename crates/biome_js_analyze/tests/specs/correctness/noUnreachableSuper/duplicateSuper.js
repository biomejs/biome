// valid
class B extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        } else {
            super(false);
        }
    }
}

// invalid
class C extends A {
    constructor() {
        super(1);
        super(2);
    }
}

// invalid
class D extends A {
    constructor(cond) {
        if (cond) {
            super(true);
        }
        super();
    }
}

// invalid
class E extends A {
    constructor(cond) {
        do {
            super();
        } while (cond);
    }
}

// invalid
class F extends A {
    constructor(condA, condB) {
        if (condA) {
            super(true);
        }
        if (condB) {
            super(true);
        }
    }
}

// invalid
class G extends A {
    constructor(condA, condB) {
        while (condA) {
            if (condB) {
                super();
            }
        }
    }
}

// invalid
class G extends A {
    constructor(condA, condB) {
        try {
            super()
        } catch {
            super()
        }
    }
}

// invalid
class G extends A {
    constructor(condA, condB) {
        try {
            super()
        } finally {
            super()
        }
    }
}
