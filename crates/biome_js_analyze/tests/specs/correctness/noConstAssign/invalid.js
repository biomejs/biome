const a = 1;
a = 2;

const b = 2, c = 43;
b = 4;
++b;
b += 45;
b--;
function f() {
	b++;
}
function f(d) {
	b++;
}
const fn = (val) => {
	val = 0;
};

const e = () => {
	try {
		foo();
	} catch (err) {
		err = 4;
	}
};

const f = (...rest) => {
	rest = 4;
};

const g = class bar {};
bar = 1;

const h = function foo() {
	foo = 1;
};

const {
	i,
	j: { l },
} = { i: 1, j: { l: 2 } };
i = 4;
l = 4;

for (const k in [1, 2]) {
	k = 4;
}

const [p, { q }] = [1, { q: 2 }];
p = 3;
q = 4;

const { r, ...rest } = s;
r = 4;
