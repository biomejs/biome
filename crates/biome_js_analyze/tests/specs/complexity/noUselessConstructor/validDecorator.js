/* should not generate diagnostics */
class A {
    constructor(@inject("foo") foo) {}
}

@autoInjectable()
class B {
    constructor(foo) {}
}
