type A<X extends string, Y = number, Z extends string | number = number> = { x: X, y: Y, z: Z }
type A<> = {}
interface A<> {}
