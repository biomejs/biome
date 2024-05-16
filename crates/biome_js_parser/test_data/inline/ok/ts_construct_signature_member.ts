type A = { new (): string; }
type B = { new (a: string, b: number) }
type C = { new <A, B>(a: A, b: B): string }
type D = { new <const T>(a: T, b: B): string }
