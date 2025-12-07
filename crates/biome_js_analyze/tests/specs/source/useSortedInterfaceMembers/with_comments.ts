interface _InterfaceWithComments {
	// This is a zebra property
	zebra: string;

	/**
	 * Apple property with JSDoc comment
	 * @example apple = 42
	 */
	apple: number;

	// Simple comment for banana
	banana: boolean; // inline comment

	/* Block comment for charlie */
	charlie: object;
}
