/* should generate diagnostics */
class InvalidMembers {
    method() {
        console.log("missing this");
    }

    get value() {
        return 1;
    }

    set value(next) {
        consume(next);
    }

    nestedFunction() {
        function inner() {
            return this;
        }

        return inner;
    }

    nestedClass() {
        class Inner {
            method() {
                return this;
            }
        }

        return Inner;
    }

    ["computed"]() {
        return 1;
    }
}
