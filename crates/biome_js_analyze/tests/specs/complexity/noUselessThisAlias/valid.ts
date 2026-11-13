/* should not generate diagnostics */
// https://github.com/biomejs/biome/issues/13125
function f() {
    const self = this;
    class Widget {
        initial: number = self.count;
        getCount = (): number => self.count;
    }
    return new Widget();
}
