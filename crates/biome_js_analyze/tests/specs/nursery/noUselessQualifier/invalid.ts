/* should generate diagnostics */

namespace TypeMember {
	export type B = number;
	const x: TypeMember.B = 3;
}

namespace ValueMember {
	export const x = 3;
	export const y = ValueMember.x;
}

namespace OuterType {
	export type T = number;
	export namespace Inner {
		const x: OuterType.T = 3;
	}
}

namespace NestedType {
	export namespace B {
		export type T = number;
		const x: NestedType.B.T = 3;
	}
}

namespace QualifiedModuleName {
	export namespace B.C {
		export type D = number;
		const x: QualifiedModuleName.B.C.D = 3;
	}
}

namespace NestedValue {
	export namespace B {
		export const x = 3;
		const y = NestedValue.B.x;
	}
}

enum EnumMember {
	B,
	C = EnumMember.B,
}

namespace EnumInNamespace {
	export enum A {
		B,
		C = EnumInNamespace.A.B,
	}
}

// Only the namespace part of the chain is unnecessary: `B` isn't an enclosing namespace.
namespace PartialChain {
	export namespace B {
		export type T = number;
	}
	const x: PartialChain.B.T = 3;
}

namespace CalledFunction {
	export function f() {}
	export function g() {
		CalledFunction.f();
		CalledFunction.f.call(undefined);
	}
}

namespace Assignment {
	export let counter = 0;
	export function increment() {
		Assignment.counter = Assignment.counter + 1;
		Assignment.counter++;
	}
}

namespace TypeofQuery {
	export const value = 1;
	type Value = typeof TypeofQuery.value;
}

namespace ImportEquals {
	export namespace Inner {
		export const value = 1;
	}
	import Alias = ImportEquals.Inner;
}

namespace NestedFunction {
	export interface Options {}
	function create(): NestedFunction.Options {
		return {};
	}
}

// The fix keeps the comments around the reference.
namespace Comments {
	export const value = 1;
	const a = /* before */ Comments.value /* after */;
}

// The fix isn't offered when it would remove a comment.
namespace CommentInside {
	export const value = 1;
	const a = CommentInside /* inside */.value;
}
