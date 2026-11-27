// An object wrapped in `as` or `satisfies` still counts as an object last
// argument, so the chain stays on one line and only the object breaks.
reply.code(409).send({ ok: false, error: "User with this email already exists here", code: "DUPLICATE_EMAIL" } as const);

reply.code(409).send({ ok: false, error: "User with this email already exists", code: "DUP" } satisfies Body);
