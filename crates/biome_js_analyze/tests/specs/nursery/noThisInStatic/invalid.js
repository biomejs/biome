class A {
    static foo() {
        doSomething()
    }

    static bar() {
        this.foo()
    }

    static hello() {
        this.faux()
    }
    
    faux() { 
        return
    }
}

class B extends A {
    static foo() {
        super.foo()
    }

    faux() { 
        return
    }
}