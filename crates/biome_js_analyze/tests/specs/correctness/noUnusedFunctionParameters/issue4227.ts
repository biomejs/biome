class Foo {
    bar(name: string, _class: new (name: string) => any) {
        return name
    }
}