/* should not generate diagnostics */
import { ref, shallowRef, type Ref } from "vue";
import { ref as notVueRef } from "./ref";

const a = shallowRef(0);
const b = ref<number>();
const c = ref<number>(0);
const d: Ref<number | undefined> = ref();
const e = ref(0);
const f: Ref<number | null> = ref(null);
const g = ref(null) as Ref<number | null>;
const h = ref() satisfies Ref<number | undefined>;
const i = <Ref<number | undefined>>shallowRef();
const j = (ref() as Ref<number | undefined>);
const k = ref(...[0]);
const l = notVueRef();

function useA() {
	return {
		count: ref<number>(),
	};
}

let m: Ref<number | undefined>;
m = ref();

function useB(count: Ref<number | undefined>) {
	count = ref();
}

function useC(count: Ref<number | null>) {}
useC(ref(null));

const useD = (count: Ref<number | undefined>) => {};
useD(ref());

const useE = function (count: Ref<number | undefined>) {};
useE((ref()));

function useF(...counts: Ref<number | undefined>[]) {}
useF(ref(), ref());

function useG(count: Ref<number | undefined> = ref()) {}

function useH(this: Window, count: Ref<number | undefined>) {}
useH(ref());

function shadowed() {
	const ref = () => {};
	ref();
}

declare function useI(count: Ref<number | undefined>): void;
useI(ref());

class Store {
	count: Ref<number | undefined> = ref();
	optional?: Ref<number | undefined> = ref();
}
