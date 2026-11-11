/* should generate diagnostics */

class TypeWrappers extends React.Component<Props, State> {
	update() {
		this.state!.foo = "bar";
		(this.state as any).foo = "bar";
		(this.state satisfies State).foo = "bar";
		(this.state.foo as string) = "bar";
		this.state.foo! = "bar";
	}
}

class Generic<T, U extends T> extends React.Component<Props<T>, State<U>> {
	public override update(): void {
		this.state.foo = "bar";
	}
}

class TypedDelete extends React.Component<Props, State> {
	update() {
		delete this.state!.foo;
		delete (this.state as any).foo;
		delete (this.state.foo as any);
	}
}
