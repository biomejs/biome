/* should not generate diagnostics */
abstract class Base {
    abstract method(): void;
}

class Derived extends Base {
    override method() {}
    override property = () => {};
}
