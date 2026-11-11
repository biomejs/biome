/* should generate diagnostics */
declare function decorate(key: unknown): ParameterDecorator;

function makeClass(this: { key: string }) {
    class DecoratorOnly {
        method(@decorate(this.key) value: unknown) {
            return value;
        }
    }
    return DecoratorOnly;
}

function makeRestClass(this: { key: string }) {
    class RestDecoratorOnly {
        method(@decorate(this.key) ...values: unknown[]) {
            return values;
        }
    }
    return RestDecoratorOnly;
}
