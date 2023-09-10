class A {
    method() {
        type A = this;
    }
    predicate(): this is string {
        return typeof this === "string"
    }
}
