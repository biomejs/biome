class A { constructor(private x, protected y, public z) {} }
class B { constructor(readonly w, private readonly x, protected readonly y, public readonly z) {} }
class C { constructor(private x: string, readonly y?, z = "default", ...rest) {} }
