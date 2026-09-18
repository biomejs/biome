/* should not generate diagnostics */

export interface Schema<T> {
	safeParse(input: unknown): { data?: T; success: boolean };
}

export declare const schema: Schema<string>;
