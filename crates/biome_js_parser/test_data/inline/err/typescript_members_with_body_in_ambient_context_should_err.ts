declare class Test {
    constructor() {}
    name() {}
    get test(): string { return ""; }
    set test(v) {}
}
declare namespace n {
     class Test {
         constructor() {}
         name() {}
         get test(): string { return ""; }
         set test(v) {}
     }
}
