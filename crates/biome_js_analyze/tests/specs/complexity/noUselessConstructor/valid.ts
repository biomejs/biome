declare class A { constructor(options: any) }

class B {
    constructor(private name: string) {}
}

class C {
    constructor(public name: string) {}
}

class D {
    constructor(protected name: string) {}
}

class E {
    private constructor() {}
}

class F {
    protected constructor() {}
}

class G extends B {
    private constructor(foo, bar) {
      super(bar);
    }
}

class H extends B {
    constructor(name = "default") {
      super(name);
    }
}
