class Base {
    accessor value = 0;
}

class Derived extends Base {
    accessor override value = 1;
    method() {}
}

export { Derived };
