/* should not generate diagnostics */
{
	using resource = createResource();
}

async function main() {
	await using connection = await openConnection();
}
main();

for (using item of items) {}

for (await using item of items) {}

{
	using used = createResource();
	console.log(used);
}
