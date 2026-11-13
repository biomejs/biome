/* should not generate diagnostics */
const { a, b } = this;
const [c, d] = this;
const property = this.property;
const firstItem = this[0];
const object = { property: this };
foo.bar = this;

function f() {
    const self = this;
    return function distinctThisScope() {
      self.g();
    }
}

function f() {
    const self = this;
    function f() {
        self.g();
    }
    return () => {
        self.g();
    }
}

function f() {
    let self = this;
    self = {}
    return () => {
        self.g();
    }
}

function f() {
    let self;
    return () => {
        self.g();
    }
}

// https://github.com/biomejs/biome/issues/13125
const store = {
    count: 42,
    makeWidget() {
        const self = this;
        class Widget {
            initial = self.count;
            getCount = () => self.count;
        }
        return new Widget();
    },
};

function f() {
    const self = this;
    class Widget {
        static initial = self.count;
    }
    return new Widget();
}

class Class {
    a = this;
    #priv = this;

    constructor() {
      this.b = this;
      this.c = [this];
    }

    act(self = this) {
        self.f()
    }

    f() {}
}
