// Type and value merging
export type Order = -1 | 0 | 1;

interface Order {
	f(): void;
}

class Order {
	prop: number;
}

enum Order {
	Lower = -1,
	Equal = 0,
	Upper = 1,
}
