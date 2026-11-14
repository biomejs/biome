/* should generate diagnostics */

function computedFieldName() {
    const self = this;
    return class {
        [self.key] = 1;
    };
}

function computedFieldNameArrow() {
    const self = this;
    return class {
        [(() => self.key)()] = 1;
    };
}

class Widget {
    value = function () {
        const self = this;
        return () => self.value;
    }();
}
