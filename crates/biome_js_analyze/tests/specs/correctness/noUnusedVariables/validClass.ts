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
