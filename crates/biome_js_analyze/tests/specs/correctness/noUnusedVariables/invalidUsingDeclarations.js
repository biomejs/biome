/* should generate diagnostics */
{
	using resource = createResource();
}

async function main() {
	await using connection = await openConnection();
}
main();
