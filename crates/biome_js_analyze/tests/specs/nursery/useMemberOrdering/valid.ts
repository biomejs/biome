/* should not generate diagnostics */

// Accessibility modifiers with correct order
class WithAccessibility {
    public static count: number;
    protected static _id: number;
    private static _secret: string;
    public name: string;
    protected age: number;
    private _password: string;
    #hidden: number;
    constructor() {}
    method() {}
}

// Abstract class
abstract class AbstractBase {
    abstract name: string;
    constructor() {}
    abstract method(): void;
}

// Index signature
class WithIndexSignature {
    [key: string]: unknown;
    name: string;
    constructor() {}
    method() {}
}

// Static block
class WithStaticBlock {
    static count: number;
    name: string;
    constructor() {}
    method() {}
    static {
        WithStaticBlock.count = 0;
    }
}

// Readonly property
class WithReadonly {
    static readonly MAX: number = 100;
    readonly id: number;
    constructor() {}
    method() {}
}
