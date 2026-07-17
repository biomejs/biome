/* should generate diagnostics */
interface Service {
    run(): void;
}

class ServiceImpl implements Service {
    run() {}

    property = () => {};

    protected protectedMethod() {}

    protected protectedProperty = () => {};

    private privateMethod() {}

    private privateProperty = () => {};

    #privateName() {}

    #privateField = () => {};
}
