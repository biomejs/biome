<>
<Foo>{'Hello world'}</Foo>

<Foo foo={'bar'} />

<Foo foo=<Bar /> />

<Foo>{
	'Hello world'
}</Foo>

<Foo>{/*comment*/'Hello world'/*comment*/}</Foo>

<Foo>{x}{'y'}{z}</Foo>

<Foo foo={' '}></Foo>

{/* https://github.com/biomejs/biome/issues/7320 */}
<ConnectedFieldTemplate
	description1=
		'Upload a CSV file containing an "email" column, and optional "first_name" and "last_name" columns'
	description2={
		'Upload a CSV file containing an "email" column, and optional "first_name" and "last_name" columns'
	}
	className={
		"d-block"
	}
/>
</>
