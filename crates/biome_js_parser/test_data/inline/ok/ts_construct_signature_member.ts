type A = { new (): string; }
type B = { new (a: string, b: number) }
type C = { new <A, B>(a: A, b: B): string }
