/* should not generate diagnostics */

class Typed extends React.Component<Props, State> {
	override state: State = { foo: "bar" };

	static override update(): void {
		this.state = {};
	}
}

class TypedConstructor extends React.Component<Props, State> {
	constructor(props: Props) {
		super(props);
		this.state = { foo: "bar" };
		(this.state as State).foo = "baz";
	}
}
