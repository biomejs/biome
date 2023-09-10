class TestClass { #member = true; method() { delete func(this.#member) } }
class TestClass { #member = true; method() { delete [this.#member] } }
class TestClass { #member = true; method() { delete { key: this.#member } } }
class TestClass { #member = true; method() { delete (() => { this.#member; }) } }
class TestClass { #member = true; method() { delete (param => { this.#member; }) } }
class TestClass { #member = true; method() { delete (async () => { this.#member; }) } }
