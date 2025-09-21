interface MixedMembersInterface {
	zProperty: string;
	aProperty: number;
	(): void;  // Call signature - non-sortable
	yMethod(): boolean;
	bMethod(): string;
	[key: string]: any;  // Index signature - non-sortable
	cField: object;
	new (): any;  // Construct signature - non-sortable
	aField: boolean;
}