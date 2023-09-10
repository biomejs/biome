let a = "test";
type B = typeof a;
type T21 = typeof Array<string>;
type A<U> = InstanceType<typeof Array<U>>;
