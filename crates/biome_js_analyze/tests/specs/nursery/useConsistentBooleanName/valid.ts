/* should not generate diagnostics */

const isCompleted: boolean | undefined = true;
const isNullable: boolean | null = null;
const anything: any = true;
const unknownValue: unknown = true;
const referenced: Flag = true;
const isReferenced: Flag = "yes";
const isAsserted = value as boolean;
function isCompletedFunction(): boolean {
	return getValue();
}
const isArrow = (): boolean => true;
async function isReady(): Promise<boolean> {
	return getValue();
}
function isString(value: unknown): value is string {
	return typeof value === "string";
}
function assertString(value: unknown): asserts value is string {}
function download(shouldShowProgress: boolean) {}
declare function isAmbient(): boolean;
declare function getTitle(): string;

// Type-only signatures aren't checked
type Callback = (completed: boolean) => void;
interface Handler {
	(completed: boolean): void;
	handle(completed: boolean): void;
	completed: boolean;
}
declare function signature(completed: boolean): void;
abstract class Base {
	abstract run(completed: boolean): void;
}

// Overloads with prefixed names
function isOverloaded(value: string): boolean;
function isOverloaded(value: number): boolean;
function isOverloaded(value: unknown): boolean {
	return true;
}

const isSatisfied = true satisfies boolean | string;
const isValueSatisfied = value satisfies boolean;

// Default export overloads with prefixed names
export default function isDefault(value: string): boolean;
export default function isDefault(value: unknown): boolean {
	return true;
}

const shouldRetry: ((error: Error) => boolean) | undefined = getHandler();
function validate(isValid: ((value: string) => boolean) | null) {}
