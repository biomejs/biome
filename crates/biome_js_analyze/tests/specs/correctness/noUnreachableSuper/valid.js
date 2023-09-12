class B extends A {
    constructor() {
        super();
        if (foo) {
            if (bar) {
                return;
            } else {
                return;
            }
        }
    }
}

class B extends A {
    constructor() {
        super();
        this.prop = 0;
        super.method();
    }
}