interface _TestInterface {
	aProperty: string;
	[index: number]: any; // Non-sortable index signature
	bProperty: number; // This should trigger an error - sortable after non-sortable
	(): void; // Call signature
	cProperty: boolean; // This should also trigger an error
}
