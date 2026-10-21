/* should generate diagnostics */
import { ref, shallowRef } from "vue";
import * as Vue from "vue";
import { ref as createRef } from "vue";
import { useCount } from "./use-count";

const a = ref();
const b = ref(null);
const c = ref(undefined);
const d = shallowRef();
const e = (ref());
const f = createRef();
const g = Vue.ref();
const h = ref()!;

function useA() {
	const count = ref();
	return { count };
}

function useB() {
	return {
		count: ref(),
	};
}

let i;
i = ref();

function untyped(count) {}
untyped(ref());

useCount(ref());

composable.useCount(ref());

function typedFirst(count: Ref<number | undefined>) {}
typedFirst(ref(), ref());

function withThis(this: Window, count) {}
withThis(ref());

const { j } = { j: ref() };

class Store {
	count = ref();
}

class OptionalStore {
	count? = ref();
}
