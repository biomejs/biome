
class Test {
    name: string;

    constructor(
                         a: string
    ) {}

    declare private readonly test?: string;
    readonly test2!: string;

    display2(): void { console.log(this.name); }
    my_name(): string {
        return this.name
    };

    public get my_name2(): any { return this.name; }
    public set my_name2(name) { this.name = name; }



}

class Test2 {

    static readonly [a: string]: string;
}


abstract class A {}
abstract class ConcreteMembers {
    name: string;



    constructor(name: string) { this.name = name; }



    display(): void { console.log(this.name); }
    public get my_name() { return this.name; }
    public set my_name(name) { this.name = name; }
    #private_method() { }
}
abstract class AbstractMembers {
    abstract name: string;
    abstract display();
    abstract get my_name();
    abstract set my_name(val);
}


abstract class Test1 {
                private c?: string;
                private d?: string;
                private readonly e: string;
                private readonly f: string;
                protected abstract readonly g: string;
                protected readonly abstract h: string;
}