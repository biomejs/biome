interface A<Prop> { prop: Prop }
interface B extends A<string> {}
interface C extends A<number>, B {}
