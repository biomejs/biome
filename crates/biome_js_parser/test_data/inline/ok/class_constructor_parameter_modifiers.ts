class Base { name!: string; other!: string }
class Sub extends Base {
 constructor(private priv: string, protected prot: string, public pub: string, override name: string, readonly read: string, protected override readonly other: string) {
     super();
 }
}
