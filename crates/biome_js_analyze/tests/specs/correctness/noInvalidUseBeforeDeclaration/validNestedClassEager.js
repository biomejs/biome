/** should not generate diagnostics */

// A nested class expression in an instance field initializer runs when an
// instance is created, so the classes it refers to are already declared.
class Outer {
	Inner = class extends Later {};
}
class Later {}