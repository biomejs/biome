// The object would also break in the expanded chain, so the chain stays on one
// line and only the object breaks.
reply.code(409).send({ ok: false, error: "User with this email already exists here", code: "DUPLICATE_EMAIL" });

z.string().min(2).max(200).regex(/[a-z]/gm, { message: "Only English alphabet symbols and hyphen allowed" });
