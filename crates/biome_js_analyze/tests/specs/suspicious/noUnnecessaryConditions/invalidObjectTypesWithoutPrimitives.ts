// should generate diagnostics
// No primitive and no function has the member `id`, so a falsy test and the
// tests of `typeof` below remove `Config` and `Point`.

interface Config {
	id: number;
}

class Point {
	id = 0;
}

function falsyInterface(value: Config | undefined) {
	if (!value) {
		if (value) {
		}
	}
}

function falsyClass(value: Point | null) {
	if (!value) {
		if (value) {
		}
	}
}

function typeofUndefined(value: Config | undefined) {
	if (typeof value === "undefined") {
		if (value) {
		}
	}
}

function typeofFunction(value: Config | (() => void)) {
	if (typeof value === "function") {
		if (value) {
		}
	}
}
