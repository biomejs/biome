
type Cheating<T extends 1> = T extends 1 ? Promise<string> : Promise<string>;

async function promiseLike(): Cheating<1> {
	return new Promise((res, _rej) => res('yep'));
}

promiseLike();
