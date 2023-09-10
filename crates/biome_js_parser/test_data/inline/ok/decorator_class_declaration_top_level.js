@decorator
class Foo { }
@first.field @second @(() => decorator)()
class Bar {}
