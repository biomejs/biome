// should not generate diagnostics

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

export function genericWithDefault<T extends FormValues>({
	inert = false,
}: GenericProps<T>) {
	return inert || undefined;
}

export function plainWithDefault({ inert = false }: PlainProps) {
	return inert || undefined;
}

export function renamedWithDefault({ inert: renamed = false }: PlainProps) {
	return renamed || undefined;
}

declare const props: PlainProps;
const { inert: fromDeclarator = false } = props;
export const declaratorResult = fromDeclarator || undefined;

declare const list: (number | undefined)[];
const [first = 1] = list;
export const arrayResult = first || undefined;
