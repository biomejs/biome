<Hello
	lastName="Smith"
	firstName="John"
/>;
<Hello tel={5555555} address="NY" {...this.props} lastName="Smith" firstName="John" />;
<Hello a10="" a9="" A="" />;

/* Nested JSX: both outer and inner attributes are unsorted (issue #9884) */
<Outer
	z="x"
	a={
		<Inner
			z={1}
			a={2}
		/>
	}
/>;
