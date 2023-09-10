class C {
    constructor(@foo readonly x: number) {}
}
class CC {
    constructor(@foo @dec(arg) readonly x: number) {}
}
class CC {
    constructor(@foo @dec.method(arg) readonly x: number) {}
}
class CCC {
    constructor(@foo @dec.method(arg) private readonly x: number) {}
}
