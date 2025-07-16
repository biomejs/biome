/* should not generate diagnostics */

class Foo {
    private readonly version = "42";

    public bar(): void {
        // @ts-ignore
        const x = {
            version: this.version,
        };
    }
}
