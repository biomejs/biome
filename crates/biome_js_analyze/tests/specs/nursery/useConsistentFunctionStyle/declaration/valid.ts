// should not generate diagnostics
const decoratedMethod = () => class {
    @decorate(this)
    method() {}
};
const decoratedGetter = () => class {
    @decorate(this)
    get value() { return 0; }
};
const decoratedSetter = () => class {
    @decorate(this)
    set value(value: number) {}
};
const decoratedProperty = () => class {
    @decorate(this)
    field = this;
};
const decoratedStaticProperty = () => class {
    @decorate(this)
    static field = this;
};
const wrappedThis = (() => this) as () => unknown;
const typedWrapper: () => void = (() => {}) as () => void;
const wrappedSequence = ((register(), (() => {}) as () => void))!;
