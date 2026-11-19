// should generate diagnostics

const A = "a";

function stillMissing(kind: "a" | "b" | "c") {
	if (kind === A) return;
	switch (kind) {
		case "b":
			break;
	}
}
