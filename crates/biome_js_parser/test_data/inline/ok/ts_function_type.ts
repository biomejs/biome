type A = () => string;
type B = (a: string) => string;
type C = (b = "test") => string;
type D = (c, d) => string
type E = ([a]) => string
type F = ({a}) => string
type G = <A, B>(a: A, b: B) => string
type H = (a: any) => a is string;
type I = ({ a, b }?) => string;
