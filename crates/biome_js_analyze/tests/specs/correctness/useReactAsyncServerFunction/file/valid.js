/* should not generate diagnostics */
'use server';

export async function serverFunction1() {
	// ...
}
export const serverFunction2 = async function () {
	// ...
}
export const serverFunction3 = async () => {
	// ...
}
