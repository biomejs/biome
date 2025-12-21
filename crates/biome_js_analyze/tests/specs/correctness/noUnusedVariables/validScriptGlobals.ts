/* should not generate diagnostics */
interface Array<T> {
	g: (a: T) => void;
}

// Namespaces can export without making this a module.
namespace M {
	export const n = 1;
}