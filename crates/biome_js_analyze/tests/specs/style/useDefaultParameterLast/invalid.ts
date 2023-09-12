export function h(a: number, b?: number, c: number) {}

export function l(a = 0, b?: number, c: number) {}

export function f(a = 0, b = 0, c?: string, c: string) {}

export function h(a/* before */?/* after */: number, b: number) {}

export class Foo {
    constructor(readonly a = 10, readonly b: number) {}
}