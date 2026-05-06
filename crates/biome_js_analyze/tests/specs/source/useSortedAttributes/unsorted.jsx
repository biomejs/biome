<Hello
	lastName="Smith"
	firstName="John"
/>;
<Hello tel={5555555} address="NY" {...this.props} lastName="Smith" firstName="John" />;
<Hello a10="" a9="" A="" />;

// https://github.com/biomejs/biome/issues/9884
// Both outer and a nested JSX-valued attribute have unsorted attributes.
<Outer z="x" a={<Inner z={1} a={2} />} />;
