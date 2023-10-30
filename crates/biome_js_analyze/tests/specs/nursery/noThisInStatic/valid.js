function foo() { this }
() => { this }
class A { constructor() { this } }
class A { foo() { this } }
class A { static foo() { function foo() { this } } }