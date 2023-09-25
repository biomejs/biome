class A {
    static foo() {
        doSomething()
    }

    static bar() {
        this.foo()
    }

    fax() {
        return 'asd';
    }
}

class B extends A {
    static foo() {
        super.foo()
    }

    fax() { 
        return 'asd';
    }
}