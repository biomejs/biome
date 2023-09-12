export class Foo1 {
  constructor(readonly name: string) {}
}

export class Foo2 {
  constructor(private name: string) {}
}

export class Foo3 {
  constructor(protected name: string) {}
}

export class Foo4 {
  constructor(public name: string) {}
}

export class Foo5 {
  constructor(private readonly name: string) {}
}

export class Foo6 {
  constructor(protected readonly name: string) {}
}

export class Foo7 {
  constructor(public readonly name: string) {}
}