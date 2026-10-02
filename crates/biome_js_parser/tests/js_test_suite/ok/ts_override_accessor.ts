class Base {
    accessor value = 0;
}

class Derived extends Base {
    override accessor value = 1;
}

export { Derived };
