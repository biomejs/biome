// With `expand: "never"`, objects don't keep the line break of the source, so
// the chain expands and the object breaks whether or not the source object
// spans multiple lines.
reply.code(409).send({ ok: false, error: "User with this email already exists here", code: "DUPLICATE_EMAIL" });

reply.code(409).send({
	ok: false,
	error: "User with this email already exists here",
	code: "DUPLICATE_EMAIL",
});
