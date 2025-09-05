/* should not generate diagnostics */

<>
	<Foo
		ref={(ref) => {
			this._div = ref;
		}}
	/>
	<Foo ref={this._refCallback.bind(this)} />
</>;
