declare namespace shared {
	type State = string;
}

declare const shared: {
	useState(): shared.State;
};

export = shared;
