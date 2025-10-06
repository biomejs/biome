// should generate diagnostics
<>
  <div>{"test"}</div>
	<>test</>
	<>
		<div>
			{'asdjfl'}
			{'test'}
			{'foo'}
		</div>
	</>
	<Foo bar={"bar"} />
	<Foo bar="test">
		{'Test' + name}
	</Foo>
	<Foo bar={`${baz}`} />
	<Foo bar={"first" + "second"} />
	<Foo bar={"third" + binding} />
	<Foo bar={binding + "fourth"} />
</>
