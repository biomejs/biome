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
    abstract set my_age(age,);
}
