// valid
export type EventHandler<T extends string> = `on${T}`
export type EventHandlerDefault<T extends string = 'click'> = `on${T}`

export type NestedContext<S extends NestedContext<''>> = '' | `(${S})`
export type NestedContextDefault<S extends NestedContextDefault = ''> = '' | `(${S})`

export type Whatever<S extends number> = `Hello ${S}`
export type WhateverDefault<S extends number = 2> = `Hello ${S}`

// Const assertions are valid
const fruits = ["banana"] as const;

class X {
  f() {
    this.g;
    type T1 = typeof this.g;
    type T2 = X['g'];
  }

  g() {
  }
}

// Invalid
export type Invalid<S extends number> = `Hello ${T}`
