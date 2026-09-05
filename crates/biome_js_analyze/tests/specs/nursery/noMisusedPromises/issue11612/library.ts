/* should not generate diagnostics */

export interface Schema<T> {
	refine(check: (value: T) => boolean): Schema<T>;
}

export declare const schema: Schema<{
	confirmPassword: string;
	password: string;
}>;
