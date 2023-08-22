const self = this, v = 0, /*u*/ u = 2, self2 = this;

function f() {
    // assignment comment
    const self = this;
    return () => {
        /*a*/self/*b*/.g();
    }
}

function f() {
    let self = this;
    return () => {
        self.g();
    }
}

function f() {
    var self;
    self = this;
    self = this;
    return () => {
        self.g();
    }
}
