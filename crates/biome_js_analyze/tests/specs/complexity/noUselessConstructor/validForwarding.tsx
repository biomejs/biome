/* should not generate diagnostics */
class Base<T> {
    constructor(public value: T) {}
}

class Narrowed extends Base<string | number> {
    constructor(value: string) {
        super(value);
    }
}
