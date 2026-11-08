/* should not generate diagnostics */

class InitialState extends React.Component {
	constructor(props) {
		super(props);
		this.state = { foo: "bar" };
		this.state.count = 1;
		if (props.name) {
			this.state.name = props.name;
		}
		this.state.value = compute();
		delete this.state.draft;
	}
}

class NestedComponentInConstructor extends Component {
	constructor() {
		super();
		class AnotherComponent extends Component {
			constructor() {
				super();
			}
		}
		this.state = {};
	}
}

class ClassField extends React.Component {
	state = { foo: "bar" };
}

class UsesSetState extends React.Component {
	update() {
		this.setState({ foo: this.state.foo });
		const value = this.state.foo;
		return value;
	}
}

class OtherMembers extends React.Component {
	#state = 0;

	update(object) {
		this.#state = 1;
		this.stateful = 1;
		this.props.state = 1;
		this.user.state.foo = 1;
		object.state = 1;
		object.state.foo = 1;
		this[key] = 1;
		delete this.stateful;
		delete this.props.state;
		delete this.user.state.foo;
		delete object.state.foo;
		delete this[key];
		typeof this.state.foo;
		void this.state;
		!this.state.foo;
	}
}

class DifferentThis extends React.Component {
	update() {
		function regularFunction() {
			this.state.foo = "bar";
		}

		const functionExpression = function () {
			this.state.foo = "bar";
		};

		const object = {
			method() {
				this.state.foo = "bar";
			},
			get value() {
				this.state.foo = "bar";
				return 1;
			},
			set value(next) {
				this.state.foo = next;
			},
		};

		class Nested {
			method() {
				this.state.foo = "bar";
			}
		}
	}
}

class StaticMembers extends React.Component {
	static instance = (this.state = {});

	static {
		this.state = {};
	}

	static update() {
		this.state = {};
	}

	static get value() {
		this.state = {};
		return 1;
	}

	static set value(next) {
		this.state = next;
	}
}

class NotAComponent {
	update() {
		this.state.foo = "bar";
		this.state = {};
		delete this.state.foo;
	}
}

class OtherBase extends Base {
	update() {
		this.state.foo = "bar";
	}
}

function FunctionComponent() {
	const state = {};
	state.foo = "bar";
	this.state = {};
	return <div />;
}

// Not detected: these changes don't assign through `this.state`.
class UndetectedChanges extends React.Component {
	update(item) {
		this.state.items.push(item);
		const that = this;
		that.state.foo = "bar";
		this.getState().foo = "bar";
	}
}
