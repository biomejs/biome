// should generate diagnostics

type FormValues = {
	value: string;
};

type GenericProps<T extends FormValues> = {
	values: T;
	inert?: boolean;
};

type PlainProps = {
	inert?: boolean;
};

export function genericWithoutDefault<T extends FormValues>({
	inert,
}: GenericProps<T>) {
	return inert || undefined;
}

export function plainWithoutDefault({ inert }: PlainProps) {
	return inert || undefined;
}

export function nullableWithDefault({
	value = "fallback",
}: {
	value?: string | null;
}) {
	return value || undefined;
}

declare const list: (number | undefined)[];
const [head] = list;
export const arrayResult = head || undefined;
