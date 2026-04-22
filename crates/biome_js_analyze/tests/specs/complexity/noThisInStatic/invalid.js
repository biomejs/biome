export default class B extends A {
    static { this.CONSTANT += super.foo(); }

    static CONSTANT = this.OTHER_CONSTANT;
    static OTHER_CONSTANT = super.ANOTHER_CONSTANT;

    static get property() {
        /*before*/this/*after*/;
        return /*before*/super/*after*/.x;
    }

    static set property(x) {
        () => this;
        () => super.x = x;
    }

    static method() {
        return this.CONSTANT + super.ANOTHER_CONSTANT;
    }
}

class C extends A {
    static method() {
        return this.CONSTANT + super.ANOTHER_CONSTANT;
    }
}

const D = class D extends f() {
    static method() {
        return this.CONSTANT + super.ANOTHER_CONSTANT;
    }
}


const E = class extends f() {
    static method() {
        return this.CONSTANT + super.ANOTHER_CONSTANT;
    }
}

// Polymorphic static method: replacing `this` with the base class name
// would break subclass behavior at runtime
class Base {
    constructor(public name) {}

    static create(name) {
        return new this(name);
    }
}

class Sub extends Base {
    greet() {
        return `Hi, ${this.name}`;
    }
}
