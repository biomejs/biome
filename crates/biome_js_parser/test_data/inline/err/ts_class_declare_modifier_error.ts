class Test {
    declare method(): string;
    declare constructor(declare readonly prop) {}
    declare get test() { return "a" }
    declare set test(value: string) {}
    declare [name: string]: string;
    declare accessor foo: string;
}
