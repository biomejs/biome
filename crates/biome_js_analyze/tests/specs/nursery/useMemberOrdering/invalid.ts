/* should generate diagnostics */

// method before property
class MethodBeforeProperty {
    method(): void {}
    name: string;
}

// constructor before static property
class ConstructorBeforeStatic {
    constructor() {}
    static count: number;
}

// instance property before static property
class InstanceBeforeStatic {
    name: string;
    static count: number;
}

// private method before constructor
class PrivateMethodBeforeConstructor {
    private doSomething(): void {}
    constructor() {}
}

// complex accessibility violation
class ComplexViolation {
    public method(): void {}
    protected _name: string;
    private _id: number;
    constructor() {}
}

// getter before property
class GetterBeforeProperty {
    get value(): number { return 0; }
    name: string;
}

// method before constructor
class MethodBeforeConstructor {
    method(): void {}
    constructor() {}
}
