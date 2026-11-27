/* should not generate diagnostics */
declare function decorate(key: unknown): ParameterDecorator;

function makeClass(this: { key: string }) {
    class DecoratorAndDefault {
        fallback = 1;

        method(@decorate(this.key) value = this.fallback) {
            return value;
        }
    }
    return DecoratorAndDefault;
}
