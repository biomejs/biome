function foo<
	Lorem,
	Ipsum,
>(
	a: Lorem,
	b?: Ipsum,
): Bar<
	Lorem
> {
	return a as unknown as Bar<
		// comment inside an erased range
		Lorem
	>;
}

interface Multi {
	a: string;
	b: number;
}

class Klass<
	T,
>
	implements
		Iface,
		Other
{
	value: Map<
		string,
		T
	> = new Map();
}
