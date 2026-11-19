// should generate diagnostics

const EMPTY = "";
const NONE = -1;
const A = "a";

function afterNegativeComparison(index: number | null) {
	if (index === -1) return index?.toFixed();
}

function afterConstantComparison(value: string | null) {
	if (value === EMPTY) return value?.length;
}

function afterVoidComparison(value: string | undefined) {
	if (value !== void 0) return value?.length;
}

function afterTemplateComparison(value: string | null) {
	if (value === `a`) return value?.length;
}

function caseOfConstant(value: string | null) {
	switch (value) {
		case EMPTY:
			return value?.length;
	}
}

function caseOfNegative(value: number | null) {
	switch (value) {
		case NONE:
			return value?.toFixed();
	}
}

function afterExhaustiveSwitchWithConstants(kind: "a" | "b", label: string | null) {
	switch (kind) {
		case A:
			if (label === null) return;
			break;
		case "b":
			if (label === null) return;
	}
	return label?.length;
}
