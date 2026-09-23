// https://github.com/biomejs/biome/issues/10531
// Fastify-style reproduction at lineWidth 100: a chain whose final object
// argument is wide enough that it wouldn't fit on its own line either, so the
// stable layout keeps the chain inline and breaks only the object. The inline
// and pre-formatted forms must produce identical output (idempotent).
function handler(reply) {
	return reply.code(409).send({ ok: false, error: "User with this email already exists here", code: "DUPLICATE_EMAIL" });
}

function handlerPreformatted(reply) {
	return reply.code(409).send({
		ok: false,
		error: "User with this email already exists here",
		code: "DUPLICATE_EMAIL",
	});
}
