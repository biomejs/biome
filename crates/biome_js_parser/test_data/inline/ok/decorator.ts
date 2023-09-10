// class expressions
let a = @decorator class {};
let b = @decorator @functionDecorator(1,2,3) class {};
let c = @first @second class Foo {}
// class declarations
@decorator class Foo {};
@decorator @functionDecorator(1,2,3) class Bar {};
@first @second class Baz {}
// abstract class declarations
@decorator abstract class Foo {};
@decorator @functionDecorator(1,2,3) abstract class Bar {};
@first @second abstract class Baz {}
// exported class declarations
export @decorator class Foo {};
export @decorator @functionDecorator(1,2,3) class Bar {};
export @first @second class Baz {}
@decorator
export class Foo { }
@first.field @second @(() => decorator)()
export class Bar {}
@before
export @after class Foo { }
@before.field @before @(() => decorator)()
export @after.field @after @(() => decorator)() class Bar {}
