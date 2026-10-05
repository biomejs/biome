class Base {
    accessor value = 0;
}

abstract class Derived extends Base {
    accessor readonly field = 1;
    override accessor readonly value = 1;
    private static accessor readonly other: string;
    protected abstract accessor readonly name: string;
    method() {}
}
