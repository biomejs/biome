// should not generate diagnostics
// TypeScript ignores assertion functions declared as variables without a type
// annotation (TS2775), so `value` keeps its declared type after the call.
import { useQuery } from "@tanstack/react-query";

const assertUndefined = (value: unknown): asserts value is undefined => {
	if (value !== undefined) {
		throw new Error();
	}
};

declare function getCached(): string | undefined;

useQuery({
	queryKey: ["cached"],
	queryFn: () => {
		const value = getCached();
		assertUndefined(value);
		return value;
	},
});
