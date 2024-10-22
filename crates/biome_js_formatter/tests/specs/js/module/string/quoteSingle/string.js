import hey from "hey"
import hey from "hey";
import "x" with { type: "json" }
import "foo" with { "type": "json" };
import foo from "foo.json" with { type: "json" };
import foo from "foo.json" with {

    type:
        "json" };
import foo2 from "foo.json" with { "type": "json", type: "html", "type": "js" };
import a, * as b from "foo"

const foo = {};

foo["bar"] = true;
foo["foo-bar"] = true;
foo.bar["bar"]["lorem_ispsum"].foo["lorem-ipsum"] = true;

a[ b ]
c?.[ d ]

let a = { // leading comment
  "type": "bar"
  // trailing comment
}

class Foo extends Boar {
	static { // some comment
		this.a = "test";
	}

	method() {
		return "ipsum";
	}

	static staticMethod() {
		return "bar"
	}
}

export * from "hey"

export * as something_bad_will_happen from "something_bad_might_not_happen"

export * as something_bad_will_happen from "something_bad_might_not_happen" with { "type": "json", "type2": "json3"}


// this one should switch to use single quotes
("content '' \"\"\" ");

// this one should switch to use double quotes
('content \'\' " ');

// you should keep all the character as they are
("content \\' \\' ");

// you should remove the escape
("content \'\' ")
