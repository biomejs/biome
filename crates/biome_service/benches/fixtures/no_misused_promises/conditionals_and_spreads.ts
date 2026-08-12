const pending = Promise.resolve(true);

if (pending) {
}

const selected = pending ? 1 : 0;

while (pending) {
	break;
}

do {
} while (pending);

const load = () => Promise.resolve(true);

if (load()) {
}

console.log({ ...pending });
console.log([...pending]);
[1, 2, 3].filter(() => pending);

if (await pending) {
}

void selected;
