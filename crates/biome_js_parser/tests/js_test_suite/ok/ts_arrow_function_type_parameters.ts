let a = <A, B extends A, C = string>(a: A, b: B, c: C) => "hello";
let b = async <A, B>(a: A, b: B): Promise<string> => "hello";
