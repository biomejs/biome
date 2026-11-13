/* should generate diagnostics */

class AssignProperty extends React.Component {
	update() {
		this.state.foo = "bar";
	}
}

class ReplaceState extends React.Component {
	update() {
		this.state = {};
	}
}

class NestedProperty extends React.Component {
	update() {
		this.state.person.name.first = "bar";
		this.state.person.name.last = "baz";
	}
}

class ComputedKeys extends React.Component {
	update(index) {
		this["state"] = {};
		this.state["foo"] = "bar";
		this.state.list[index].name = "bar";
	}
}

class Operators extends React.Component {
	update() {
		this.state.count += 1;
		this.state.name ??= "bar";
		this.state.count++;
		--this.state.count;
	}
}

class Destructuring extends React.Component {
	update(list, object) {
		[this.state.first] = list;
		({ name: this.state.name } = object);
		for (this.state.item of list) {
		}
	}
}

class NamedImport extends Component {
	update() {
		this.state.foo = "bar";
	}
}

class Pure extends PureComponent {
	update() {
		this.state.foo = "bar";
	}
}

class Pure2 extends React.PureComponent {
	update() {
		this.state.foo = "bar";
	}
}

const Expression = class extends React.Component {
	update() {
		this.state.foo = "bar";
	}
};

export default class extends React.Component {
	update() {
		this.state.foo = "bar";
	}
}

class ClassMembers extends React.Component {
	handleClick = () => {
		this.state.foo = "bar";
	};

	get value() {
		this.state.foo = "bar";
		return 1;
	}

	set value(next) {
		this.state.foo = next;
	}
}

class ArrowCallback extends React.Component {
	componentDidMount() {
		load().then(() => {
			this.state.foo = "bar";
		});
	}
}

class Lifecycle extends React.Component {
	componentDidUpdate() {
		this.state.foo = "bar";
	}

	shouldComponentUpdate() {
		this.state.foo = "bar";
	}

	componentWillUnmount() {
		this.state.foo = "bar";
	}
}

class ConstructorCallback extends React.Component {
	constructor(props) {
		super(props);
		doSomethingAsync(() => {
			this.state = "bad";
		});
	}
}

class ConstructorArrow extends React.Component {
	constructor(props) {
		super(props);
		const update = () => {
			this.state.foo = "bar";
		};
		this.update = update;
	}
}

class ConstructorCallArgument extends React.Component {
	constructor(props) {
		super(props);
		register((this.state = {}));
	}
}

class Generic extends React.Component {
	constructor() {
		super();
		someFn();
	}

	someFn() {
		this.state.foo = "bar";
	}
}

class DeleteOperator extends React.Component {
	update() {
		delete this.state;
		delete this.state.foo;
		delete this.state.person.name;
		delete this.state["foo"];
		delete this.state?.foo;
	}
}

class ConstructorDeleteCallback extends React.Component {
	constructor(props) {
		super(props);
		load(() => {
			delete this.state.foo;
		});
	}
}
