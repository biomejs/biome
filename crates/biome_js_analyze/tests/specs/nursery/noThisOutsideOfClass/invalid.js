/* should generate diagnostics */

this.value;
const getValue = () => this.value;

function Foo(value) {
    this.value = value;
}

const functionExpression = function () {
    return this.value;
};

const object = {
    method() {
        return this.value;
    },
    get value() {
        return this._value;
    },
    set value(value) {
        this._value = value;
    },
};

Foo.prototype.method = function () {
    this.value();
};

new SDK({
    onReady: function () {
        this.value;
    },
});

class NestedFunctions {
    method() {
        function getValue() {
            return this.value;
        }

        return getValue();
    }

    field = function () {
        return this.value;
    };

    static {
        function update() {
            this.value = 1;
        }

        update();
    }
}

class ComputedMembers {
    [this.methodName]() {}
    [this.propertyName] = 1;
}

class Derived extends this.Base {}

(function () {
    this.initialize();
})();

const objectFunction = {
    getName: function () {
        return this.name;
    },
};
