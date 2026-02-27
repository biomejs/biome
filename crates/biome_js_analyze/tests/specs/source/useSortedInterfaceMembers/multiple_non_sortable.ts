interface _UnsortedInterface {
	zProperty: string;
	aProperty: number;
	new (): any;  // Construct signature
	(): void;  // Call signature
	yMethod(): boolean;
	[index: number]: string;  // Index signature with number
	bMethod(): string;
	[key: string]: any;  // Index signature with string
	cField: object;
}
