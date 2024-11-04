@decorator
export class Foo { }
@first.field @second @(() => decorator)()
export class Bar {}
@before
export @after class Foo { }
 @before
 export abstract class Foo { }
 @before
 export @after abstract class Foo { }
