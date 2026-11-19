/* should generate diagnostics */

type Kind = ("a" & string) | "b";

function missingIntersectedLiteral(kind: Kind) {
	switch (kind) {
		case "b":
			break;
	}
}
