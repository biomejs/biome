/* should not generate diagnostics */

function instanceField() {
    const self = this;
    return class {
        value = self.value;
    };
}

function instanceFieldArrow() {
    const self = this;
    return class {
        getValue = () => self.value;
    };
}

function staticField() {
    const self = this;
    return class {
        static value = self.value;
    };
}

function staticFieldArrow() {
    const self = this;
    return class {
        static getValue = () => self.value;
    };
}
