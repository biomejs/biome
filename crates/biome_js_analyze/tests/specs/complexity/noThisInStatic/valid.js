/* should not generate diagnostics */
function foo() { this }
() => { this }
class A { constructor() { this } }
class A { foo() { this } }
class A { static foo() { function foo() { this } } }

class Base {
    static create(name) {
        return new this(name);
    }

    static field = new this();

    static {
        new this();
    }
}

class Sub extends Base {}
Sub.create("Alice");
