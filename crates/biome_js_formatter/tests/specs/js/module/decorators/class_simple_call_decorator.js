@decorator.method(value) export class Foo {}
@decorator.method(value)
export class Foo {}
@decorator.method(value) @decorator2.method(value) export class Foo {}
@decorator.method(value)
@decorator2.method(value) export class Foo {}

@decorator.method(value) @decorator2.method(value)
export class Foo {}

export @decorator.method(value) class Foo {}
export @decorator.method(value)
class Foo {}
export
@decorator.method(value) class Foo {}

export @decorator.method(value) @decorator2.method(value) class Foo {}
export @decorator.method(value)
@decorator2.method(value) class Foo {}
export @decorator.method(value)
@decorator2.method(value) class Foo {}
export
@decorator.method(value) @decorator2.method(value) class Foo {}

export
@decorator.method(value)
@decorator2.method(value) class Foo {}

export
@decorator.method(value)
@decorator2.method(value)
class Foo {}

@decorator.method(value)
export @decorator2.method(value) class Foo {}

@decorator.method(value) export @decorator2.method(value) class Foo {}

@decorator.method(value) export
@decorator2.method(value) class Foo {}

@decorator.method(value)
export
@decorator2.method(value) class Foo {}

@decorator.method(value) @decorator2.method(value) export @decorator3.method(value) @decorator4.method(value) class Foo {}
export @decorator.method(value) @decorator2.method(value)  @decorator3.method(value) @decorator4.method(value) class Foo {}

