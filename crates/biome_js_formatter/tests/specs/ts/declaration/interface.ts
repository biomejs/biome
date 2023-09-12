interface A {}
interface B extends A /** comment **/ {
    something: string
}


interface C<Foo, Bar, Loreum, Ipsum, ItShouldBeLongEnoughToCoverSomeEdgeCases, But, Maybe, Not> extends B {
    something: string
}

// @ts-ignore
interface D extends B<string, symbol>, F<string, symbol>, G<string, number, symbol>, H<string, number, symbol> {
    something1: string,
    something2: string,
    something3: string,
    something4: string,
    something5: string,
}
// @ts-ignore
interface D extends B<string, symbol>, F<string, symbol> {

}

interface Wrong {
	aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
	ppppppp: ppppppp
}

f(() => {
	interface Wrong {
		aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
		ppppppp: ppppppp
	}
});

x.y(() => {
	interface Wrong {
		a: {
			b: string;
		};
		p: {
			q: string;
		};
	}
});
