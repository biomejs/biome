/* should generate diagnostics */

class Foo {
    accessor [this.key] = 1;
}

const object = {
    validator(this: TrackedModel) {
        function getValue() {
            return this.value;
        }

        return getValue();
    },

    method() {
        return this.value;
    },
};

function validator(value: TrackedModel) {
    return this.value;
}

const arrow = (this: TrackedModel) => this.value;
