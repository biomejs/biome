/* should generate diagnostics */
import {
	defineRule,
	nodeKinds,
	nodeKinds as kinds,
	tokenKinds,
} from "@biomejs/runtime/plugin";
import * as plugin from "@biomejs/runtime/plugin";

const CALLS = nodeKinds("JS_CALL_EXPRESSION");
const IMPORTS = nodeKinds("JS_IMPORT_CALL_EXPRESSION");
const BOTH = CALLS.union(IMPORTS);

export const rule = defineRule({
	run(node) {
		const local = nodeKinds("JS_CALL_EXPRESSION");
		tokenKinds("EQ2", "NEQ");
		kinds("JS_NEW_EXPRESSION");
		plugin.nodeKinds("JS_NEW_EXPRESSION");
		CALLS.union(IMPORTS);
		BOTH.union(IMPORTS);
		CALLS.union(nodeKinds("JS_NEW_EXPRESSION"));
		nodeKinds("JS_CALL_EXPRESSION").union(IMPORTS);
		local.union(IMPORTS);
	},
});

function helper() {
	return nodeKinds("JS_CALL_EXPRESSION");
}
const arrow = () => tokenKinds("IDENT");
(function () {
	nodeKinds("JS_CALL_EXPRESSION");
})();
function withDefault(set = nodeKinds("JS_CALL_EXPRESSION")) {}

class Visitor {
	kinds = nodeKinds("JS_CALL_EXPRESSION");
	constructor() {
		this.other = nodeKinds("JS_NEW_EXPRESSION");
	}
	get calls() {
		return nodeKinds("JS_CALL_EXPRESSION");
	}
	static create() {
		return nodeKinds("JS_CALL_EXPRESSION");
	}
}
const object = {
	method() {
		return nodeKinds("JS_CALL_EXPRESSION");
	},
};
function makeClass() {
	return class {
		static KINDS = nodeKinds("JS_CALL_EXPRESSION");
	};
}
