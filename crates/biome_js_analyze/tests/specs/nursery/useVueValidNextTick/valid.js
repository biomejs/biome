/* should not generate diagnostics */
import { nextTick as vueNextTick } from "vue";
import { nextTick } from "some-library";

// Not Vue's `nextTick`.
nextTick();
nextTick;
other.nextTick();
vm.$nextTick();
process.nextTick(callback, argument);

function shadowed(vueNextTick) {
	vueNextTick();
}

class Component {
	tick = vueNextTick;
}

function withDefault(tick = vueNextTick) {
	return tick;
}

new Promise(vueNextTick);
