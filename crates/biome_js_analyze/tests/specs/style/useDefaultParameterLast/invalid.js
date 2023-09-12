export function f(a = 0, b) {}

export function g(a, b = 0, c) {}

export function g(a, b /* before */ = /* mid */ 0/* after */) {}

export function g(a, b /* before */ = /* mid */ 0 /* after */ /* after comma */, c) {}

export function u(a=5, b, c) {}
