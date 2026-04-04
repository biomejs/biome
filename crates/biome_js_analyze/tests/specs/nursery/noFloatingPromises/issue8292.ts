/* should generate diagnostics */

class AsyncClass {
    async returnsPromise() {
        return 'value';
    }
}

class ShouldBeReported {
    constructor(public field: AsyncClass) { }

    async shouldBeReported() {
        this.field.returnsPromise();
    }
}
