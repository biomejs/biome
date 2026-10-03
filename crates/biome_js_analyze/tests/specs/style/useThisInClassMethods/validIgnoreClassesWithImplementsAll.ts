/* should not generate diagnostics */
interface Service {
    run(): void;
}

class ServiceImpl implements Service {
    run() {}
    property = () => {};

    get value() {
        return 1;
    }

    set value(next: number) {
        consume(next);
    }
}
