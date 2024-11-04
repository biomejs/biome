class C { #hasOwnProperty; foo() { Object.#hasOwnProperty.call(obj, prop) } }
class C { #call; foo() { Object.hasOwnProperty.#call(obj, prop) } }
class C { #hasOwnProperty; foo() { Object.prototype.#hasOwnProperty.call(obj, prop) } }
class C { #call; foo() { Object.prototype.hasOwnProperty.#call(obj, prop) } }
class C { #hasOwnProperty; foo() { ({}.#hasOwnProperty.call(obj, prop)) } }
class C { #call; foo() { ({}.hasOwnProperty.#call(obj, prop)) } }
class C { #prototype; foo() { Object.#prototype.hasOwnProperty.call(obj, prop) } }