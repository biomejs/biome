/* should not generate diagnostics */

class A {
    constructor(a: number) {
        console.log(a)
    }
    f(a: number) {
        console.log(a)
    }
}
console.log(new A(1));

// we never flag class expressions
new (class B { })

// a, b, c, and d are instance properties (declared as property parameters)
class C {
	constructor(private a, public b, protected c, readonly d) {}
}
console.log(new C(1, 2, 3, 4));

export let Outside;
class D {
    static {
        Outside = D;
    }
}

class D {
    static {
        new D();
    }

    constructor() { console.log("Built") }
}

class D {
    static {
        D.p;
    }

    static get p() { console.log("access"); return; }
}

class D {
    static {
        D["p"];
    }

    static get p() { console.log("access"); return; }
}
