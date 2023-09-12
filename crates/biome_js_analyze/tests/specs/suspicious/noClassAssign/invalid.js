function case12() {
	class A { }
	A = 0;
}

function case13() {
	class B { }
	({B} = 0);
}

function case14() {
	class C { }
	({b: C = 0} = {});
}

function case15() {
	D = 0;
	class D { }
}

function case16() {
	class E {
		b() {
			E = 0;
		}
	}
}

function case17() {
	let F = class F {
		b() {
			F = 0;
		}
	}
}

function case18() {
	class G { }
	G = 0;
	G = 1;
}
