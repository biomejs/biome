/* should not generate diagnostics */
class WithFields {
    field = () => this.value;
    other = function () {
        return this.value;
    };

    accessor fieldAccessor = () => this.value;

    static ignored = () => {};
    nonCallable = 1;
}
