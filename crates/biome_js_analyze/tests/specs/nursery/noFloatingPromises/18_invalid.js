class Foo {
    get foo() {
        if (!this.initialised) {
            this.initialise();
            return "foo";
        }

        return "foo";
    }

    async initialise() {
        // Do stuff
    }
}
