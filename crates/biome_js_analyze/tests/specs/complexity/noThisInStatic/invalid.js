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

class FactoryCases {
    static method() {
        new this(this);
        new Foo(this);
        new this.Factory();
    }
}

class BaseWithHasInstance {
    constructor(name) {
        this.name = name;
    }

    static [Symbol.hasInstance](instance) {
        if (
            instance === null ||
            instance === undefined ||
            typeof instance !== "object"
        ) {
            return false;
        }

        if (
            this.prototype &&
            Object.prototype.isPrototypeOf.call(this.prototype, instance)
        ) {
            return true;
        }

        let proto = Object.getPrototypeOf(instance);
        while (proto !== null) {
            if (proto.constructor?.name === this.name) {
                return true;
            }
            proto = Object.getPrototypeOf(proto);
        }

        return false;
    }
}

class HasInstanceSub1 extends BaseWithHasInstance {}
class HasInstanceSub2 extends BaseWithHasInstance {}

const hasInstanceSub1 = new HasInstanceSub1("sub1");
hasInstanceSub1 instanceof HasInstanceSub2;
