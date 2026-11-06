/* should generate diagnostics */
import { nextTick } from "vue";

export async function update() {
	(nextTick as any)();
	nextTick()!;
	(nextTick as typeof nextTick).then(callback);
	return nextTick(callback);
}

export const tick = () => nextTick(callback);

function shadowedPromise(Promise) {
	Promise.all([nextTick(), other]);
}
