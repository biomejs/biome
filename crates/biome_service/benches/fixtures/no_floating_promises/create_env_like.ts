interface Environment {
	SERVER: boolean;
}

interface Schema {
	nonempty(): Schema;
}

interface EnvOptions {
	extends: unknown[];
	runtimeEnv: Record<string, unknown>;
	server: Record<string, Schema>;
	skipValidation: boolean;
}

declare const processEnv: Record<string, string | undefined>;
declare const schema: { string(): Schema };
declare const shared: {
	runtimeEnv: Record<string, unknown>;
	server: Record<string, Schema>;
};
declare const preset: () => unknown;
declare const production: string;
declare const staging: string;
declare const skipValidation: boolean;
declare function requiredIn(
	schema: Schema,
	environments: string[],
): Schema;
declare function createEnv<T = Environment>(options: EnvOptions): T;

createEnv({
	extends: [
		preset(),
		preset(),
		preset(),
		preset(),
		preset(),
		preset(),
		preset(),
		preset(),
	],
	runtimeEnv: {
		...shared.runtimeEnv,
		ENV_0: processEnv.ENV_0,
		ENV_1: processEnv.ENV_1,
		ENV_2: processEnv.ENV_2,
		ENV_3: processEnv.ENV_3,
		ENV_4: processEnv.ENV_4,
		ENV_5: processEnv.ENV_5,
		ENV_6: processEnv.ENV_6,
		ENV_7: processEnv.ENV_7,
		ENV_8: processEnv.ENV_8,
	},
	server: {
		...shared.server,
		ENV_0: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_1: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_2: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_3: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_4: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_5: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_6: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_7: requiredIn(schema.string().nonempty(), [production, staging]),
		ENV_8: requiredIn(schema.string().nonempty(), [production, staging]),
	},
	skipValidation,
});
