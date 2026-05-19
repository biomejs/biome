/* should generate diagnostics */
abstract class Base {
    abstract method(): void;
}

class Derived extends Base {
    override method() {}

    protected protectedMethod() {}

    private privateMethod() {}
}

interface Service {
    run(): void;
}

class ServiceImpl implements Service {
    run() {}
}
