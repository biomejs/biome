/* should not generate diagnostics */
export function SomeComponent() {
	return <div x-some-prop={<>Foo</>} x-another-prop={<Fragment>foo</Fragment>} />;
}
