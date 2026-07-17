/* should not generate diagnostics */
class ValidMembers {
    constructor() {
        sideEffect();
    }

    method() {
        return this.value;
    }

    withArrow() {
        const getValue = () => this.value;
        return getValue();
    }

    withSuper() {
        return super.toString();
    }

    static method() {
        return 1;
    }

    static {
        this.ready = true;
    }
}
