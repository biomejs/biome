class A {
    static foo() {
        doSomething()
    }

    static bar() {
        A.foo()
    }

    fax() {
        return 'asd';
    }
}

class B extends A {
    static foo() {
        A.foo()
    }

    fax() { 
        return 'asd';
    }
}